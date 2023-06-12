//! Python code generator.

use std::collections::{HashMap, HashSet};

use super::generator::{DynamicAsset, Generator, Manifest, StaticAsset};
use crate::descriptor::{DataPackage, DataResource, TableSchema, TableSchemaField};
use convert_case::{Case, Casing};
use rust_embed::RustEmbed;
use serde::Serialize;
use tinytemplate::TinyTemplate;

pub struct Python<'a> {
    pub data_package: &'a DataPackage,
    tt: TinyTemplate<'a>,
}

const PYTHON_VERSION: &str = "0.1.0";

#[derive(RustEmbed)]
#[folder = "static/python/0.1.0/"]
struct Asset;

// Helpers.
struct FieldData {
    /// The field name, unchanged from the `DataPackage`.
    field_name: String,
    /// The Python class name.
    field_class: String,
    code: String,
}

/// Standardizes the import path by stripping off any `.py` suffix.
fn standardize_import(path: String) -> String {
    if path.ends_with(".py") {
        path.strip_suffix(".py").unwrap().to_string()
    } else {
        path
    }
}

static IMPORT_TEMPLATE_NAME: &'static str = "imports";
static IMPORT_TEMPLATE: &'static str = "
from field import field_classes
from field_expr import FieldExpr
from table import Table
";

static FIELD_DEF_TEMPLATE_NAME: &'static str = "field_def";
static FIELD_DEF_TEMPLATE: &'static str = "{field_ref}: type(\"{field_name}\", (), {})";

static TABLE_CLASS_TEMPLATE_NAME: &'static str = "table";
static TABLE_CLASS_TEMPLATE: &'static str = "
{imports}

// Import the dataset.
from {dataset_path} import {dataset_ref}

class {class_name}:
    # Source path.
    sourcePath = \"{resource_path}\"

    class Map(dict):
        __getattr__ = dict.get

    // Fields.
    fields = Map({
        {field_defs}
    })

    # Singleton.
    instance = None
    table_ = None

    def __init__(self):
        self.table_ = Table(
            dataset={dataset_ref},
            name=\"{resource_name}\",
            source=\"{resource_path}\",
            fields=list({class_name}.fields.values())
        )
    
    @classmethod
    def get() -> {class_name}:
        if not {class_name}.instance:
            {class_name}.instance = {class_name}()
        return {class_name}.instance

    @classmethod
    def table() -> Table:
        return {class_name}.get().table_

    @classmethod
    def select(*selection: {selector} | FieldExpr) -> Table:
        return {class_name}.table()[list(selection)]

    # Rest of the stuff.
{dataset_ref}.addTable({class_name}.table())
";

impl<'a> Python<'a> {
    pub fn new(dp: &'a DataPackage) -> Self {
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
        // Do not perform HTML escaping.
        tt.set_default_formatter(&tinytemplate::format_unescaped);

        Self {
            data_package: dp,
            tt,
        }
    }

    /// Returns a field's name, class, and code (key-value definition).
    fn gen_field(&self, field: &TableSchemaField) -> FieldData {
        let (field_name, field_class) = match field {
            TableSchemaField::NumberField { name, .. }
            | TableSchemaField::IntegerField { name, .. }
            | TableSchemaField::BooleanField { name, .. } => {
                (name.to_string(), String::from("Field"))
            }
            TableSchemaField::StringField { name, .. } => {
                (name.to_string(), String::from("StringField"))
            }
            TableSchemaField::DateField { name, .. } => {
                (name.to_string(), String::from("DateField"))
            }
            TableSchemaField::DateTimeField { name, .. } => {
                (name.to_string(), String::from("DateTimeField"))
            }
            _ => panic!("Unsupported field type {:?}", field),
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
    fn gen_field_defs(&self, fields: &Vec<TableSchemaField>) -> (String, Vec<String>, Vec<String>) {
        let fields_data = fields
            .iter()
            .map(|f| self.gen_field(f))
            .collect::<Vec<FieldData>>();
        let field_defs = fields_data
            .iter()
            .map(|fd| fd.code.as_str())
            .collect::<Vec<&str>>()
            .join(",\n\t");

        let field_classes: HashSet<String> = fields_data
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
    fn data_package(&self) -> &DataPackage {
        return &self.data_package;
    }

    fn resource_table(&self, r: &DataResource) -> DynamicAsset {
        let dp = self.data_package();
        let name = dp.name.as_ref().unwrap();
        let package_name = self.package_name(&name);
        let dataset_ref = self.variable_name(&package_name.as_str());
        let dataset_path = standardize_import(self.file_name(&package_name.as_str()));

        let resource_name = r.name.as_ref().unwrap();
        let resource_path = r.path.as_ref().unwrap().to_string();
        let schema = r.schema.as_ref().unwrap();
        let class_name = resource_name.to_case(Case::Pascal);
        if let TableSchema::Object { fields, .. } = schema {
            let (field_defs, field_names, field_classes) = self.gen_field_defs(fields);
            let selector = field_names
                .iter()
                .map(|n| format!("\"{n}\""))
                .collect::<Vec<String>>()
                .join(" | ");

            #[derive(Serialize)]
            struct Context {
                imports: String,
                dataset_ref: String,
                dataset_path: String,
                class_name: String,
                resource_name: String,
                resource_path: String,
                field_defs: String,
                selector: String,
            }
            let context = Context {
                imports: self.gen_imports(field_classes),
                dataset_ref,
                dataset_path,
                class_name: class_name.clone(),
                resource_name: resource_name.to_string(),
                resource_path,
                field_defs,
                selector,
            };

            let code = match self.tt.render(TABLE_CLASS_TEMPLATE_NAME, &context) {
                Ok(result) => result,
                Err(e) => panic!("Failed to render table class with error {:?}", e),
            };

            DynamicAsset {
                path: self.file_name(&class_name),
                name: class_name,
                content: code,
            }
        } else {
            panic!("String TableSchema not supported")
        }
    }

    fn version(&self) -> String {
        String::from(PYTHON_VERSION)
    }

    fn static_assets(&self) -> Vec<StaticAsset> {
        Asset::iter()
            .map(|p| StaticAsset {
                path: p.to_string(),
                content: Asset::get(&p).unwrap(),
            })
            .collect()
    }

    fn entry_file_name(&self) -> String {
        String::from("__init__.py")
    }

    fn root_dir(&self) -> String {
        String::from("python")
    }

    fn source_dir(&self) -> String {
        String::from("src")
    }

    fn variable_name(&self, name: &str) -> String {
        name.to_case(Case::Snake)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.py", name.to_case(Case::Snake))
    }

    fn package_name(&self, name: &str) -> String {
        name.to_case(Case::Kebab)
    }

    fn manifest(&self) -> Manifest {
        let dp = self.data_package();
        let name = dp.name.as_ref().unwrap();
        let pkg_name = self.package_name(name);
        let version = dp.version.to_string();
        let description = dp.description.as_ref().unwrap_or(name).to_string();

        #[derive(Serialize)]
        struct PackageJson<'a> {
            name: String,
            version: String,
            description: String,
            main: String,
            scripts: HashMap<&'a str, &'a str>,
            dependencies: HashMap<&'a str, &'a str>,
        };

        let pkg_json = PackageJson {
            name: pkg_name,
            version,
            description,
            main: String::from("./dist/__init__.py"),
            scripts: HashMap::from_iter([("build", "py"), ("prepublish", "py")]),
            dependencies: HashMap::from_iter([
                ("python", "^3.11.3"),
                ("grpcio", "^1.54.2"),
                ("protobuf", "^4.23.2"),
                ("python-graphql-client", "^0.4.3"),
            ]),
        };

        let pkg_json = match serde_json::to_string_pretty(&pkg_json) {
            Ok(res) => res,
            Err(e) => panic!("Failed to JSON serialize \"package.json\" with error {e}"),
        };

        Manifest {
            file_name: String::from("package.json"),
            description: pkg_json,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn standardize_import_works() {
        assert_eq!(standardize_import("foo/bar.py".into()), "foo/bar");
        assert_eq!(standardize_import("baz".into()), "baz");
    }
}
