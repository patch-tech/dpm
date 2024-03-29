//! Python code generator.

use std::collections::BTreeSet;

use super::generator::{exec_cmd, DynamicAsset, Generator, ItemRef, Manifest, StaticAsset};
use crate::api::GetDatasetVersionResponse;
use crate::descriptor::{FieldType, Table, TableSchema, TableSchemaField};
use convert_case::{Case, Casing};
use regress::Regex;
use rust_embed::RustEmbed;
use semver::Version;
use serde::Serialize;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use tinytemplate::TinyTemplate;

pub struct Python<'a> {
    pub dataset: &'a GetDatasetVersionResponse,
    tt: TinyTemplate<'a>,
}

const PYTHON_VERSION: &str = "0.2.1";

#[derive(RustEmbed)]
#[folder = "static/python/src"]
#[exclude = "test/*"]
struct Asset;

// Helpers.
struct FieldData {
    /// The field name, unchanged from the `Dataset`.
    field_name: String,
    /// The Python class name.
    field_class: String,
    code: String,
}

/// Standardizes the import path by stripping off any specified prefix and suffix, if they exist.
/// ```ignore
/// standardize_import(
///     &Path::new("src").join("foo").join("bar.py"),
///     Some("src".into()),
///     Some(".py".into()),
/// )
/// ```
/// returns `"foo.bar"`
///
fn standardize_import(
    path: &PathBuf,
    strip_prefix: Option<String>,
    strip_suffix: Option<String>,
) -> String {
    let strip_prefix = strip_prefix.unwrap_or("".into());
    let path = if !strip_prefix.is_empty() && path.starts_with(&strip_prefix) {
        match path.strip_prefix(&strip_prefix) {
            Ok(path) => path.to_path_buf(),
            Err(e) => {
                eprintln!(
                    "Failed to remove prefix {:?} with error {:?}",
                    strip_prefix, e
                );
                path.to_owned()
            }
        }
    } else {
        path.to_owned()
    };

    let path = path.display().to_string();
    let strip_suffix = strip_suffix.unwrap_or(".py".into());
    let path = if path.ends_with(&strip_suffix) {
        path.strip_suffix(&strip_suffix).unwrap().to_string()
    } else {
        path
    };
    // Transform the OS path string to the dot-separated Python import path.
    // E.g. "backends/factory" -> "backends.factory"
    let path = Path::new(&path).to_path_buf();
    path.components()
        .filter(|c| !matches!(c, Component::CurDir))
        .map(|c| c.as_os_str())
        .collect::<Vec<&OsStr>>()
        .join(OsStr::new("."))
        .into_string()
        .ok()
        .unwrap()
}

/// Clean the name to retain only alphanumeric, underscore, hyphen, and space characters.
fn clean_name(name: &str) -> String {
    let re = Regex::new(r"[a-zA-Z0-9_\-\ ]+").unwrap();
    re.find_iter(name)
        .map(|m| &name[m.range()])
        .collect::<Vec<&str>>()
        .join("")
}

static IMPORT_TEMPLATE_NAME: &str = "imports";
static IMPORT_TEMPLATE: &str = "
from typing import Literal, Union

from ..field import {field_classes}
from ..field_expr import FieldExpr
from ..table import Table
";

static FIELD_DEF_TEMPLATE_NAME: &str = "field_def";
static FIELD_DEF_TEMPLATE: &str = "\"{field_ref}\": {field_class}(\"{field_name}\")";

static TABLE_CLASS_TEMPLATE_NAME: &str = "table";
static TABLE_CLASS_TEMPLATE: &str = "
{imports}

class {class_name}:
    # Source path.
    source_path = \"https://example.snowflakecomputing.com\"

    class Map(dict):
        __getattr__ = dict.get

    # Fields.
    fields = Map(\\{
    {field_defs}
    })

    # Singleton.
    instance = None
    table_ = None

    def __init__(self):
        self.table_ = Table(
            package_id=\"{dataset_id}\",
            dataset_name=\"{dataset_name}\",
            dataset_version=\"{dataset_version}\",
            name=\"{resource_name}\",
            source=\"https://example.snowflakecomputing.com\",
            fields=list({class_name}.fields.values())
        )

    @classmethod
    def get(cls) -> \"{class_name}\":
        if not {class_name}.instance:
            {class_name}.instance = {class_name}()
        return {class_name}.instance

    @classmethod
    def table(cls) -> Table:
        return {class_name}.get().table_

    @classmethod
    def select(cls, *selection: Union[{selector}, FieldExpr]) -> Table:
        return {class_name}.table().select(*selection)
";

static ENTRY_POINT_TEMPLATE_NAME: &str = "entry";
static ENTRY_POINT_TEMPLATE: &str = "
{{ for item in imports }}
from {item.path} import {item.ref_name}
{{ endfor }}
";

static VERSION_TEMPLATE_NAME: &str = "version";
static VERSION_TEMPLATE: &str = "
# The version of the generated code.
CODE_VERSION = \"{code_version}\"\n
";

/// Returns a version string for a Python package:
///   dataset-version "." code-version ("a" draft-number)?
/// See: https://peps.python.org/pep-0440/#public-version-identifiers
fn package_instance_version(v: &Version) -> String {
    if v.pre.is_empty() {
        format!("{}.{}", v, PYTHON_VERSION)
    } else {
        // Assume this has form "draft.<number>", and so can be
        // joined with the rest of the string via a "a".
        let (_, draft_number) = v.pre.split_at(v.pre.find('.').unwrap());
        format!(
            "{}.{}.{}.{}a{}",
            v.major, v.minor, v.patch, PYTHON_VERSION, draft_number
        )
    }
}

impl<'a> Python<'a> {
    pub fn new(dp: &'a GetDatasetVersionResponse) -> Self {
        let mut tt = TinyTemplate::new();
        if tt
            .add_template(IMPORT_TEMPLATE_NAME, IMPORT_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", IMPORT_TEMPLATE_NAME);
        }
        if tt
            .add_template(FIELD_DEF_TEMPLATE_NAME, FIELD_DEF_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", FIELD_DEF_TEMPLATE_NAME);
        }
        if tt
            .add_template(TABLE_CLASS_TEMPLATE_NAME, TABLE_CLASS_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", TABLE_CLASS_TEMPLATE_NAME);
        }
        if tt
            .add_template(ENTRY_POINT_TEMPLATE_NAME, ENTRY_POINT_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", ENTRY_POINT_TEMPLATE_NAME);
        }
        if tt
            .add_template(VERSION_TEMPLATE_NAME, VERSION_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", VERSION_TEMPLATE_NAME);
        }
        // Do not perform HTML escaping.
        tt.set_default_formatter(&tinytemplate::format_unescaped);

        Self { dataset: dp, tt }
    }

    /// Returns a field's name, class, and code (key-value definition).
    fn gen_field(&self, field: &TableSchemaField) -> FieldData {
        let field_name = field.name.to_owned();
        let field_class = match field.type_ {
            FieldType::Number | FieldType::Boolean => String::from("Field"),
            FieldType::String => String::from("StringField"),
            FieldType::Date => String::from("DateField"),
            FieldType::Time => String::from("TimeField"),
            FieldType::DateTime => String::from("DateTimeField"),
            FieldType::Array => {
                unreachable!("Unsupported field type {:?}, please report a bug!", field)
            }
        };
        let field_ref = self.variable_name(&field_name);

        #[derive(Serialize)]
        struct Context {
            field_ref: String,
            field_class: String,
            field_name: String,
        }

        let context = Context {
            field_ref,
            field_class: field_class.clone(),
            field_name: field_name.clone(),
        };

        let code = match self.tt.render(FIELD_DEF_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render field defs with error {:?}", e),
        };

        FieldData {
            field_name,
            field_class,
            code,
        }
    }

    /// Returns a tuple: (code snippet declaring the fields map, the list of field names, and set of field classes used).
    fn gen_field_defs(&self, fields: &[TableSchemaField]) -> (String, Vec<String>, Vec<String>) {
        let fields_data = fields
            .iter()
            .map(|f| self.gen_field(f))
            .collect::<Vec<FieldData>>();
        let field_defs = fields_data
            .iter()
            .map(|fd| fd.code.as_str())
            .collect::<Vec<&str>>()
            .join(",\n\t");

        let field_classes: BTreeSet<String> = fields_data
            .iter()
            .map(|fd| fd.field_class.clone())
            .collect();

        let field_names = fields_data.iter().map(|fd| fd.field_name.to_string());
        (
            field_defs,
            Vec::from_iter(field_names),
            Vec::from_iter(field_classes),
        )
    }

    fn gen_imports(&self, field_classes: Vec<String>) -> String {
        #[derive(Serialize)]
        struct Context {
            field_classes: String,
        }

        let context = Context {
            field_classes: field_classes.join(", "),
        };
        match self.tt.render(IMPORT_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render imports with error {:?}", e),
        }
    }
}

impl Generator for Python<'_> {
    fn dataset(&self) -> &GetDatasetVersionResponse {
        self.dataset
    }

    fn resource_table(&self, r: &Table) -> DynamicAsset {
        let dp = self.dataset();
        let dataset_id = dp.uuid.to_string();
        let dataset_name = self.dataset_name(&dp.name);

        let resource_name = &r.name;
        let class_name = clean_name(resource_name).to_case(Case::Pascal);
        let TableSchema { fields, .. } = &r.schema;
        let (field_defs, field_names, field_classes) = self.gen_field_defs(fields);
        let selector = field_names
            .iter()
            .map(|n| format!("Literal[\"{n}\"]"))
            .collect::<Vec<String>>()
            .join(", ");

        #[derive(Serialize)]
        struct Context {
            imports: String,
            dataset_id: String,
            dataset_name: String,
            dataset_version: String,
            class_name: String,
            resource_name: String,
            field_defs: String,
            selector: String,
        }
        let context = Context {
            imports: self.gen_imports(field_classes),
            dataset_id,
            dataset_name,
            dataset_version: dp.version.version.to_string(),
            class_name: class_name.clone(),
            resource_name: resource_name.to_string(),
            field_defs,
            selector,
        };

        let code = match self.tt.render(TABLE_CLASS_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render table class with error {:?}", e),
        };

        let path = Path::new(&self.source_dir())
            .join("tables")
            .join(self.file_name(&class_name));
        DynamicAsset {
            path: Box::new(path),
            name: class_name,
            content: code,
        }
    }

    fn version(&self) -> DynamicAsset {
        let src_dir = self.source_dir();
        let src_dir = Path::new(&src_dir);
        #[derive(Serialize)]
        struct Context {
            code_version: String,
        }
        let context = Context {
            code_version: String::from(PYTHON_VERSION),
        };
        let code = match self.tt.render(VERSION_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render version file with error {:?}", e),
        };

        DynamicAsset {
            path: Box::new(src_dir.join("version.py")),
            name: "codeVersion".into(),
            content: code,
        }
    }

    fn static_assets(&self) -> Vec<StaticAsset> {
        Asset::iter()
            .map(|p| {
                // Prefix static source paths with this package's source directory.
                // E.g., `field.py` -> `my_pkg/field.py`.
                let src_dir = self.source_dir();
                let path = Box::new(Path::new(&src_dir).join(p.to_string()));
                StaticAsset {
                    path,
                    content: Asset::get(&p).unwrap(),
                }
            })
            .collect()
    }

    fn entry_file_name(&self) -> String {
        String::from("__init__.py")
    }

    fn root_dir(&self) -> PathBuf {
        let dp = self.dataset();
        let package_directory = format!(
            "{}@{}",
            self.dataset_name(&dp.name),
            package_instance_version(&dp.version.version)
        );
        Path::new("python").join(package_directory)
    }

    fn source_dir(&self) -> String {
        let dp = self.dataset();
        let dataset_name = self.dataset_name(&dp.name);
        dataset_name.to_case(Case::Snake)
    }

    fn variable_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Snake)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.py", name.to_case(Case::Snake))
    }

    fn dataset_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Kebab)
    }

    fn manifest(&self) -> Manifest {
        let dp = self.dataset();
        let pkg_name: String = self.dataset_name(&dp.name);
        let version = package_instance_version(&dp.version.version);

        #[derive(Serialize)]
        struct PyprojectToml<'a> {
            project: Project<'a>,
        }

        #[derive(Serialize)]
        struct Project<'a> {
            name: String,
            version: String,
            description: String,
            dependencies: Vec<&'a str>,
        }

        let project_toml = PyprojectToml {
            project: Project {
                name: pkg_name,
                version,
                description: dp.description.clone(),
                dependencies: Vec::from_iter([
                    "grpcio ~= 1.54.2",
                    "protobuf ~= 4.23.2",
                    "python-graphql-client ~= 0.4.3",
                    "python-dateutil ~= 2.8.2",
                ]),
            },
        };

        let project_toml = match toml::ser::to_string_pretty(&project_toml) {
            Ok(res) => res,
            Err(e) => panic!("Failed to TOML serialize \"pyproject.toml\" with error {e}"),
        };

        Manifest {
            file_name: String::from("pyproject.toml"),
            description: project_toml,
        }
    }

    fn entry_code(&self, imports: Vec<ItemRef>) -> DynamicAsset {
        #[derive(Serialize)]
        struct Context {
            imports: Vec<ItemRef>,
        }

        let context = Context {
            imports: imports
                .iter()
                .map(|x| ItemRef {
                    path: Box::new(
                        Path::new(&standardize_import(&x.path, None, Some(".py".into())))
                            .to_path_buf(),
                    ),
                    ref_name: x.ref_name.to_string(),
                })
                .collect(),
        };

        let content = match self.tt.render(ENTRY_POINT_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render entry point code with error {:?}", e),
        };

        DynamicAsset {
            path: Box::new(Path::new(&self.source_dir()).join(self.entry_file_name())),
            name: "".into(),
            content,
        }
    }

    /// Builds the generated package. E.g., for the `Python` target, builds the Python package using
    /// the recommended Python build tools: `virtualenv, pip`, and `python -m build`.
    fn build_package(&self, path: &Path) {
        println!("Building Python package");
        exec_cmd(
            "build virtual environment",
            path,
            "python3",
            &["-m", "venv", "venv"],
        );

        exec_cmd(
            "activate virtual environment",
            path,
            "bash",
            &[
                "-e",
                "-c",
                ". venv/bin/activate\npython3 -m pip install --upgrade pip\npip install build\npython3 -m build",
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::{api::DatasetVersion, descriptor::Dataset};

    #[test]
    fn standardize_import_works() {
        assert_eq!(
            standardize_import(
                &Path::new("./src/foo").join("bar.py"),
                Some("./src".into()),
                Some(".py".into())
            ),
            "foo.bar"
        );
        assert_eq!(
            standardize_import(&PathBuf::new().join("baz"), None, Some(".ts".into())),
            "baz"
        );
    }

    #[test]
    fn clean_name_works() {
        assert_eq!(clean_name("oneword"), "oneword");
        assert_eq!(clean_name("two W0rds"), "two W0rds");
        assert_eq!(clean_name("words, with fie;nds"), "words with fiends");
        assert_eq!(clean_name("underscores_ are_ok"), "underscores_ are_ok");
        assert_eq!(clean_name("dots.are.not"), "dotsarenot");
        assert_eq!(clean_name("dine-and-dash"), "dine-and-dash");
    }

    #[test]
    fn root_dir_works() {
        let dp = Dataset::read("tests/resources/dataset.json").unwrap();
        let res = GetDatasetVersionResponse {
            name: dp.name.to_string(),
            uuid: Uuid::from_bytes(dp.id.as_bytes().to_owned()),
            description: dp.description.unwrap_or("".into()),
            version: DatasetVersion {
                version: dp.version,
                accelerated: false,
                dataset: dp.tables,
                patch_state: None,
                patch_state_data: None,
            },
        };
        let generator = Box::new(Python::new(&res));
        let expected_dir = format!("test-snowflake@0.1.0.{}", PYTHON_VERSION);
        assert_eq!(generator.root_dir(), Path::new("python").join(expected_dir));
    }
}
