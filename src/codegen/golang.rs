//! Golang code generator

use std::collections::BTreeSet;
use super::generator::{exec_cmd, DynamicAsset, Generator, ItemRef, Manifest, StaticAsset};
use convert_case::{Casing, Case};
use regress::Regex;
use crate::descriptor::{FieldType, Table, TableSchema, TableSchemaField};
use tinytemplate::TinyTemplate;
use rust_embed::RustEmbed;
use semver::Version;
use serde::Serialize;
use crate::api::GetDatasetVersionResponse;
use std::{path::{Path, PathBuf}};

pub struct Golang<'a> {
    pub dataset: &'a GetDatasetVersionResponse,
    tt: TinyTemplate<'a>,
}

const GOLANG_VERSION: &str = "0.0.1";

#[derive(RustEmbed)]
#[folder = "static/golang/backends"]
#[exclude = "test/*"]
struct Asset;

// Helpers.
struct FieldData {
    /// The field name, unchanged from the `Dataset`.
    field_name: String,
    /// The GO struct name.
    field_class: String,
    field_def_code: String,
    field_instance_code: String,
    as_field_def_code: String,
}

/// Clean the name to retain only alphanumeric, underscore, hyphen, and space characters.
fn clean_name(name: &str) -> String {
    let re = Regex::new(r"[a-zA-Z0-9_\-\ ]+").unwrap();
    re.find_iter(name)
        .map(|m| &name[m.range()])
        .collect::<Vec<&str>>()
        .join("")

}

static FIELD_DEF_TEMPLATE_NAME: &str = "field_def";
static FIELD_DEF_TEMPLATE: &str = "{field_ref} backends.{field_class}";

static AS_FIELD_DEF_TEMPLATE_NAME: &str = "as_field_def";
static AS_FIELD_DEF_TEMPLATE: &str = "&faf.{field_ref},";

static FIELD_INSTANCE_DEF_TEMPLATE_NAME: &str = "field_instance_def";
static FIELD_INSTANCE_DEF_TEMPLATE: &str = "{field_ref}: *backends.New{field_class}(\"{field_name}\"),";

static TABLE_CLASS_TEMPLATE_NAME: &str = "table";
static TABLE_CLASS_TEMPLATE: &str = "
package tables

import (
	\"fmt\"
	\"sync\"

	\"{dataset_name}/backends\"
)

type {class_name}Fields struct \\{
	{field_defs}
}

type {class_name} struct \\{
	Table_ *backends.Table
	Fields *{class_name}Fields
}

var (
	instance *{class_name}
	once     sync.Once
)

func asTableFields(faf *{class_name}Fields) []backends.Expr \\{
	return []backends.Expr\\{
        {as_field_defs}
	}
}

func New{class_name}() *{class_name} \\{
	once.Do(func() \\{
		fields := &{class_name}Fields\\{
            {field_instance_defs}
		}
		newTable, err := backends.NewTable(
			\"{dataset_id}\",
			\"{dataset_name}\",
			\"{dataset_version}\",
			\"{resource_name}\",
			asTableFields(fields),
			nil,
			\"https://example.snowflakecomputing.com\", // Source path
			nil,
			nil,
			nil,
			1,
		)
		if err != nil \\{
			// SHould this crash or log?
			println(fmt.Sprintf(\"error creating NewTable: %v\", err))
		}
		instance = &{class_name}\\{
			Fields: fields,
			Table_: newTable,
		}
	})
	return instance
}

func (f *{class_name}) Select(selection ...interface\\{}) *backends.Table \\{
	return f.Table_.Select(selection...)
}

func Select(selection ...interface\\{}) *backends.Table \\{
	return New{class_name}().Select(selection...)
}
";

static VERSION_TEMPLATE_NAME: &str = "version";
static VERSION_TEMPLATE: &str = "
package backends
// The version of the generated code.
const CODE_VERSION = \"{code_version}\"\n
";

/// Returns a version string for a Go module:
///   dataset-version "." code-version ("-beta." draft-number)?
/// See: https://go.dev/doc/modules/version-numbers
fn package_instance_version(v: &Version) -> String {
    if v.pre.is_empty() {
        format!("v{}.{}", v, GOLANG_VERSION)
    } else {
        // Assume this has form "draft.<number>", and so can be
        // joined with the rest of the string via a "a".
        let (_, draft_number) = v.pre.split_at(v.pre.find('.').unwrap());
        format!(
            "v{}.{}.{}-{}-beta.{}",
            v.major, v.minor, v.patch, GOLANG_VERSION, draft_number
        )
    }
}

impl<'a> Golang<'a> {
    pub fn new(dp: &'a GetDatasetVersionResponse) -> Self {
        let mut tt = TinyTemplate::new();
        if tt
        .add_template(FIELD_DEF_TEMPLATE_NAME, FIELD_DEF_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", FIELD_DEF_TEMPLATE_NAME);
        }
        if tt
            .add_template(AS_FIELD_DEF_TEMPLATE_NAME, AS_FIELD_DEF_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", AS_FIELD_DEF_TEMPLATE_NAME);
        }
        if tt
            .add_template(
                FIELD_INSTANCE_DEF_TEMPLATE_NAME,
                FIELD_INSTANCE_DEF_TEMPLATE,
            )
            .is_err()
        {
            panic!("Failed to add {:?} template", FIELD_INSTANCE_DEF_TEMPLATE_NAME);
        }
        if tt.add_template(TABLE_CLASS_TEMPLATE_NAME, TABLE_CLASS_TEMPLATE)
        .is_err()
        {
             panic!("Failed to add {:?} template", TABLE_CLASS_TEMPLATE_NAME);
        }
        if tt
            .add_template(VERSION_TEMPLATE_NAME, VERSION_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", VERSION_TEMPLATE_NAME);
        }
        // Use unescaped format.
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
        let field_ref = self.variable_name(&field_name).to_case(Case::UpperCamel);

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

        let field_def_code = match self.tt.render(FIELD_DEF_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render field defs with error {:?}", e),
        };

        let as_field_def_code = match self.tt.render(AS_FIELD_DEF_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render as field defs with error {:?}", e),
        };

        let field_instance_code = match self.tt.render(FIELD_INSTANCE_DEF_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render field instance defs with error {:?}", e),
        };

        FieldData {
            field_name,
            field_class,
            field_def_code,
            as_field_def_code,
            field_instance_code,
        }
    }

    /// Returns a tuple: (code snippet declaring the fields map, the list of field names, and set of field classes used).
    fn gen_field_defs(&self, fields: &[TableSchemaField]) -> (String, Vec<String>, Vec<String>, String, String) {
        let fields_data = fields
            .iter()
            .map(|f| self.gen_field(f))
            .collect::<Vec<FieldData>>();
        let field_defs = fields_data
            .iter()
            .map(|fd| fd.field_def_code.as_str())
            .collect::<Vec<&str>>()
            .join("\n\t");

        let as_field_defs = fields_data
        .iter()
        .map(|fd| fd.as_field_def_code.as_str())
        .collect::<Vec<&str>>()
        .join("\n\t");

        let field_instance_defs = fields_data
        .iter()
        .map(|fd| fd.field_instance_code.as_str())
        .collect::<Vec<&str>>()
        .join("\n\t");
        
        let field_classes: BTreeSet<String> = fields_data
            .iter()
            .map(|fd| fd.field_class.clone())
            .collect();

        let field_names = fields_data.iter().map(|fd| fd.field_name.to_string());
        (
            field_defs,
            Vec::from_iter(field_names),
            Vec::from_iter(field_classes),
            as_field_defs,
            field_instance_defs,
        )
    }
}

impl Generator for Golang<'_> {
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
        let (field_defs, field_names, _field_classes, as_field_defs, field_instance_defs) = self.gen_field_defs(fields);
        let selector = field_names
            .iter()
            .map(|n| format!("Literal[\"{n}\"]"))
            .collect::<Vec<String>>()
            .join(", ");

        #[derive(Serialize)]
        struct Context {
            dataset_id: String,
            dataset_name: String,
            dataset_version: String,
            class_name: String,
            resource_name: String,
            field_defs: String,
            selector: String,
            as_field_defs: String,
            field_instance_defs: String,
        }
        let context = Context {
            dataset_id,
            dataset_name,
            dataset_version: dp.version.version.to_string(),
            class_name: class_name.clone(),
            resource_name: resource_name.to_string(),
            field_defs,
            selector,
            as_field_defs,
            field_instance_defs,
        };

        let code = match self.tt.render(TABLE_CLASS_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render table class with error {:?}", e),
        };

        let path = Path::new("")
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
            code_version: String::from(GOLANG_VERSION),
        };
        let code = match self.tt.render(VERSION_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render version file with error {:?}", e),
        };

        DynamicAsset {
            path: Box::new(src_dir.join("version.go")),
            name: "codeVersion".into(),
            content: code,
        }
	}

    fn static_assets(&self) -> Vec<StaticAsset> {
        Asset::iter()
            .map(|p| {
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
        String::from("")
    }

    fn root_dir(&self) -> PathBuf {
        let dp = self.dataset();
        let package_directory = format!(
            "{}@{}",
            self.dataset_name(&dp.name),
            package_instance_version(&dp.version.version)
        );
        Path::new("golang").join(package_directory)
    }

    fn source_dir(&self) -> String {
        String::from("backends")
    }

    fn variable_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Snake)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.go", name.to_case(Case::Snake))
    }

    fn dataset_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Kebab)
    }

    fn manifest(&self) -> Manifest {
        let dp = self.dataset();
        let mod_name: String = self.dataset_name(&dp.name);
        let version = package_instance_version(&dp.version.version);

        let proj_file_name = "go.mod";
        let contents = format!("
module {mod_name}
// Module version: {version} generated by Patch Data Package Manager
go 1.21.4

require (
	github.com/golang/protobuf v1.5.3 // indirect
	golang.org/x/net v0.14.0 // indirect
	golang.org/x/sys v0.11.0 // indirect
	golang.org/x/text v0.12.0 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20230822172742-b8732ec3820d // indirect
	google.golang.org/grpc v1.59.0 // indirect
	google.golang.org/protobuf v1.31.0 // indirect
)
");

        Manifest {
            file_name: String::from(proj_file_name),
            description: contents,
        }
    }

    fn entry_code(&self, _imports: Vec<ItemRef>) -> DynamicAsset {
        DynamicAsset {
            path: Box::new(Path::new(&self.source_dir()).join(self.entry_file_name())),
            name: "".into(),
            content: "".into(),
        }
    }

    /// Builds the package.
    fn build_package(&self, path: &Path) {
        println!("Building go module");
        exec_cmd(
            "build go module",
            path,
            "go",
            &["mod", "tidy"],
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
                &Path::new("./src/foo").join("bar.go"),
                Some("./src".into()),
                Some(".go".into())
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
        let generator = Box::new(Golang::new(&res));
        let expected_dir = format!("test-snowflake@v0.1.0.{}", GOLANG_VERSION);
        assert_eq!(generator.root_dir(), Path::new("golang").join(expected_dir));
    }
}
