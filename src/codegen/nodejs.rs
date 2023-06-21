//! Node.js code generator.

use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};

use super::generator::{exec_cmd, DynamicAsset, Generator, ItemRef, Manifest, StaticAsset};
use crate::descriptor::{DataPackage, DataResource, TableSchema, TableSchemaField};
use convert_case::{Case, Casing};
use regress::Regex;
use rust_embed::RustEmbed;
use serde::Serialize;
use tinytemplate::TinyTemplate;

pub struct NodeJS<'a> {
    pub data_package: &'a DataPackage,
    tt: TinyTemplate<'a>,
}

const NODEJS_VERSION: &str = "0.1.0";

#[derive(RustEmbed)]
#[folder = "static/nodejs/"]
#[exclude = "test/*"]
#[exclude = "package*.json"]
#[exclude = "node_modules/*"]
#[exclude = "jest.config.js"]
struct Asset;

struct FieldData {
    /// The field name, unchanged from the `DataPackage`.
    field_name: String,
    /// The TypeScript class name, sans any type parameter list (`<...>`).
    field_class: String,
    /// A TypeScript
    /// [FieldDefinition](https://tc39.es/ecma262/2023/#prod-FieldDefinition),
    /// initialized to an instance of the class named in `field_class` but
    /// _with_ any type parameter list.
    code: String,
}

/// Standardizes the import path by stripping off any `.ts` suffix.
fn standardize_import(
    path: &PathBuf,
    strip_prefix: Option<&str>,
    strip_suffix: Option<&str>,
) -> PathBuf {
    let strip_prefix = strip_prefix.unwrap_or("");
    let path = if !strip_prefix.is_empty() && path.starts_with(strip_prefix) {
        match path.strip_prefix(strip_prefix) {
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
    let strip_suffix = strip_suffix.unwrap_or(".ts");
    let path = if path.ends_with(&strip_suffix) {
        path.strip_suffix(&strip_suffix).unwrap().to_string()
    } else {
        path
    };
    Path::new(&path).to_path_buf()
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
import \\{ {field_classes} } from \"../field\";
import \\{ FieldExpr } from \"../field_expr\";
import \\{ Table } from \"../table\";
";

static FIELD_DEF_TEMPLATE_NAME: &str = "field_def";
static FIELD_DEF_TEMPLATE: &str = "{field_ref}: new {field_type}(\"{field_name}\")";

static TABLE_CLASS_TEMPLATE_NAME: &str = "table";
static TABLE_CLASS_TEMPLATE: &str = "
{imports}

export class {class_name} \\{
    // Source path.
    public static sourcePath: string = \"{resource_path}\";

    // Fields.
    public static fields = \\{
        {field_defs}
    };

    private table_: Table;

    // Singleton.
    private static instance: {class_name};

    private constructor() \\{
      this.table_ = new Table(\\{
        datasetName: \"{dataset_name}\",
        datasetVersion: \"{dataset_version}\",
        name: \"{resource_name}\",
        source: \"{resource_path}\",
        fields: Object.values({class_name}.fields)
      });
    }

    private static get(): {class_name} \\{
      if (!{class_name}.instance) \\{
        {class_name}.instance = new {class_name}();
      }
      return {class_name}.instance;
    }

    public static table(): Table \\{
      return this.get().table_;
    }

    public static select(...selection: ({selector} | FieldExpr)[]): Table \\{
      return this.table().select(...selection);
    }
    // Rest of the stuff.
};
";

static ENTRY_POINT_TEMPLATE_NAME: &str = "entry";
static ENTRY_POINT_TEMPLATE: &str = "
{{ for item in imports }}
export \\{ {item.ref_name} } from \"./{item.path}\";
{{ endfor }}
";

impl<'a> NodeJS<'a> {
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
        if tt
            .add_template(ENTRY_POINT_TEMPLATE_NAME, ENTRY_POINT_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", ENTRY_POINT_TEMPLATE_NAME);
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
        let (field_name, field_type, field_class) = match field {
            TableSchemaField::NumberField { name, .. } => (
                name.to_string(),
                String::from("Field<number>"),
                String::from("Field"),
            ),
            TableSchemaField::IntegerField { name, .. } => (
                name.to_string(),
                String::from("Field<number>"),
                String::from("Field"),
            ),
            TableSchemaField::BooleanField { name, .. } => (
                name.to_string(),
                String::from("Field<boolean>"),
                String::from("Field"),
            ),
            TableSchemaField::StringField { name, .. } => (
                name.to_string(),
                String::from("StringField"),
                String::from("StringField"),
            ),
            TableSchemaField::DateField { name, .. } => (
                name.to_string(),
                String::from("DateField"),
                String::from("DateField"),
            ),
            TableSchemaField::DateTimeField { name, .. } => (
                name.to_string(),
                String::from("DateTimeField"),
                String::from("DateTimeField"),
            ),
            _ => panic!("Unsupported field type {:?}", field),
        };
        let field_ref = self.variable_name(&field_name);

        #[derive(Serialize)]
        struct Context {
            field_ref: String,
            field_type: String,
            field_name: String,
        }

        let context = Context {
            field_ref,
            field_type,
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

        // Compute the set of classes used, mapping any generic uses to their
        // class, e.g., `Field<T>` is replaced with `Field`.
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

impl Generator for NodeJS<'_> {
    fn data_package(&self) -> &DataPackage {
        self.data_package
    }

    fn resource_table(&self, r: &DataResource) -> DynamicAsset {
        let dp = self.data_package();
        let name = dp.name.as_ref().unwrap();
        let dataset_name = self.package_name(name);
        let dataset_version = dp.version.to_string();

        let resource_name = r.name.as_ref().unwrap();
        let resource_path = r.path.as_ref().unwrap().to_string();
        let schema = r.schema.as_ref().unwrap();
        let class_name = clean_name(resource_name).to_case(Case::Pascal);
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
                dataset_name: String,
                dataset_version: String,
                class_name: String,
                resource_name: String,
                resource_path: String,
                field_defs: String,
                selector: String,
            }
            let context = Context {
                imports: self.gen_imports(field_classes),
                dataset_name,
                dataset_version,
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

            let path = Path::new(self.source_dir().as_str())
                .join("tables")
                .join(self.file_name(&class_name));
            DynamicAsset {
                path: Box::new(path),
                name: class_name,
                content: code,
            }
        } else {
            panic!("String TableSchema not supported")
        }
    }

    fn version(&self) -> String {
        String::from(NODEJS_VERSION)
    }

    fn static_assets(&self) -> Vec<StaticAsset> {
        Asset::iter()
            .map(|p| StaticAsset {
                path: Box::new(PathBuf::new().join(p.to_string())),
                content: Asset::get(&p).unwrap(),
            })
            .collect()
    }

    fn entry_file_name(&self) -> String {
        String::from("index.ts")
    }

    fn root_dir(&self) -> String {
        String::from("nodejs")
    }

    fn source_dir(&self) -> String {
        String::from("src")
    }

    fn variable_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Camel)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.ts", clean_name(name).to_case(Case::Snake))
    }

    fn package_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Kebab)
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
            types: String,
            scripts: HashMap<&'a str, &'a str>,
            dependencies: HashMap<&'a str, &'a str>,
        }

        let pkg_json = PackageJson {
            name: pkg_name,
            version,
            description,
            main: String::from("./dist/index.js"),
            types: String::from("./dist/index.d.ts"),
            scripts: HashMap::from_iter([("build", "tsc"), ("prepublish", "tsc")]),
            dependencies: HashMap::from_iter([
                ("typescript", "^5.0.4"),
                ("@types/node", "^18.16.1"),
                ("@grpc/grpc-js", "^1.1.0"),
                ("@grpc/proto-loader", "^0.5.0"),
                ("google-protobuf", "^3.0.0"),
                ("graphql-request", "^6.0.0"),
                ("uuid", "^9.0.0"),
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

    fn entry_code(&self, imports: Vec<ItemRef>) -> DynamicAsset {
        #[derive(Serialize)]
        struct Context {
            imports: Vec<ItemRef>,
        }

        let src_dir = self.source_dir();
        let src_dir = Path::new(&src_dir);

        let context = Context {
            imports: imports
                .iter()
                .map(|x| ItemRef {
                    path: Box::new(standardize_import(
                        &x.path,
                        Some(&src_dir.display().to_string()),
                        Some(".ts"),
                    )),
                    ref_name: x.ref_name.to_string(),
                })
                .collect(),
        };

        let content = match self.tt.render(ENTRY_POINT_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render entry point code with error {:?}", e),
        };

        let path = src_dir.join(self.entry_file_name());
        DynamicAsset {
            path: Box::new(path),
            name: "".into(),
            content,
        }
    }

    /// Builds the generated package. E.g., for the `NodeJS` target, builds the npm package using
    /// the recommended NodeJS build commands: `npm install`, and `npm run build`.
    fn build_package(&self, path: &Path) {
        exec_cmd("install npm package", path, "npm", &["install"]);
        exec_cmd("build npm package", path, "npm", &["run", "build"]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standardize_import_works() {
        assert_eq!(
            standardize_import(
                &Path::new("src").join("foo").join("bar.ts"),
                Some("src"),
                Some(".ts")
            ),
            Path::new("foo").join("bar")
        );
        assert_eq!(
            standardize_import(&PathBuf::new().join("baz"), None, Some(".ts")),
            Path::new("baz")
        );
    }

    #[test]
    fn clean_name_works() {
        assert_eq!(clean_name("oneword"), "oneword");
        assert_eq!(clean_name("two W0r9s"), "two W0r9s");
        assert_eq!(clean_name("words, with fie;nds"), "words with fiends");
        assert_eq!(clean_name("underscores_ are_ok"), "underscores_ are_ok");
        assert_eq!(clean_name("dots.are.not"), "dotsarenot");
        assert_eq!(clean_name("dine-and-dash"), "dine-and-dash");
    }
}
