//! Csharp code generator.

use super::generator::{exec_cmd, DynamicAsset, Generator, ItemRef, Manifest, StaticAsset};
use crate::api::GetDatasetVersionResponse;
use crate::descriptor::{Table, TableSchema, TableSchemaField};
use convert_case::{Case, Casing};
use regress::Regex;
use rust_embed::RustEmbed;
use semver::Version;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tinytemplate::TinyTemplate;

pub struct Csharp<'a> {
    pub dataset: &'a GetDatasetVersionResponse,
    tt: TinyTemplate<'a>,
}

const CSHARP_VERSION: &str = "0.1.1";

#[derive(RustEmbed)]
#[folder = "static/csharp/Dpm"]
#[exclude = "Dpm.csproj"] // The project file will be written as a manifest.
#[exclude = "proto/*"] // RustEmbed fails on symbolic links; handled by embedding original proto below.
#[exclude = "test/*"]
#[exclude = "bin/*"]
#[exclude = "obj/*"]
#[exclude = ".vs/*"]
struct Asset;

// Embed the proto files available at the top-level proto directory.
#[derive(RustEmbed)]
#[folder = "proto"]
#[prefix = "proto/"]
struct ProtoAsset;

// Helpers.
struct FieldData {
    field_type_decl: String,
    field_init: String,
    field_ref: String,
}

struct FieldSnippets {
    fields_types: String,
    fields_inits: String,
    fields_list: String,
}

/// Clean the name to retain only alphanumeric, underscore, hyphen, and space characters.
fn clean_name(name: &str) -> String {
    let re = Regex::new(r"[a-zA-Z0-9_\-\ ]+").unwrap();
    re.find_iter(name)
        .map(|m| &name[m.range()])
        .collect::<Vec<&str>>()
        .join("")
}

static FIELD_INIT_TEMPLATE_NAME: &str = "field_init";
static FIELD_INIT_TEMPLATE: &str = "{field_ref}: new {field_type}(\"{field_name}\")";

static TABLE_CLASS_TEMPLATE_NAME: &str = "table";
static TABLE_CLASS_TEMPLATE: &str = "
using Dpm;

namespace {namespace} \\{
  public class {class_name} \\{
    // Fields.
    public record FieldsRecord(
        {fields_types}
    );
    public static FieldsRecord Fields = new FieldsRecord(
        {fields_inits}
    );

    private Table table_;

    // Singleton.
    private static readonly Lazy<{class_name}> lazy = new Lazy<{class_name}>(() => new {class_name}());

    private {class_name}() \\{
      this.table_ = new Table(
        packageId: \"{dataset_id}\",
        datasetName: \"{dataset_name}\",
        datasetVersion: \"{dataset_version}\",
        name: \"{resource_name}\",
        fields: new FieldExpr[] \\{
            {fields_list}
        }
      );
    }

    private static {class_name} Instance \\{ get \\{ return lazy.Value; } }

    public static Table Table() \\{
      return {class_name}.Instance.table_;
    }

    public static Table Select(params FieldExpr[] selection) \\{
      return {class_name}.Table().Select(selection);
    }
  };
}
";

static VERSION_TEMPLATE_NAME: &str = "version";
static VERSION_TEMPLATE: &str = "
namespace Dpm \\{
    public static class Constants \\{
        public const string CODE_VERSION = \"{code_version}\";
    }
}
";

/// Returns a version string for a C# package:
///   dataset-version "-" code-version (".draft." draft-number)?
/// See: https://learn.microsoft.com/en-us/nuget/concepts/package-versioning
fn package_version(v: &Version) -> String {
    if v.pre.is_empty() {
        format!("{}-{}", v, CSHARP_VERSION)
    } else {
        format!(
            "{}.{}.{}-{}.{}",
            v.major,
            v.minor,
            v.patch,
            CSHARP_VERSION,
            // Assume this has form "draft.<number>", and so can be
            // joined with the rest of the string via a ".".
            v.pre.as_str()
        )
    }
}

impl<'a> Csharp<'a> {
    pub fn new(dp: &'a GetDatasetVersionResponse) -> Self {
        let mut tt = TinyTemplate::new();
        if tt
            .add_template(FIELD_INIT_TEMPLATE_NAME, FIELD_INIT_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", FIELD_INIT_TEMPLATE_NAME);
        }
        if tt
            .add_template(TABLE_CLASS_TEMPLATE_NAME, TABLE_CLASS_TEMPLATE)
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
        // Do not perform HTML escaping.
        tt.set_default_formatter(&tinytemplate::format_unescaped);

        Self { dataset: dp, tt }
    }

    /// Returns a field's name, class, and code (key-value definition).
    fn gen_field(&self, field: &TableSchemaField) -> FieldData {
        let (field_name, field_type) = match field {
            TableSchemaField::NumberField { name, .. } => {
                (name.to_string(), String::from("Field<float>"))
            }
            TableSchemaField::BooleanField { name, .. } => {
                (name.to_string(), String::from("Field<bool>"))
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
            TableSchemaField::TimeField { name, .. } => {
                (name.to_string(), String::from("TimeField"))
            }
            TableSchemaField::ArrayField { .. } => {
                unreachable!("Unsupported field type {:?}, please report a bug!", field)
            }
        };
        let field_ref = self.variable_name(&field_name);

        #[derive(Serialize)]
        struct Context {
            field_ref: String,
            field_type: String,
            field_name: String,
        }

        let context = Context {
            field_ref: field_ref.clone(),
            field_type: field_type.clone(),
            field_name: field_name.clone(),
        };

        let field_type_decl = format!("{field_type} {field_ref}");
        let field_init = match self.tt.render(FIELD_INIT_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render field defs with error {:?}", e),
        };

        FieldData {
            field_type_decl,
            field_init,
            field_ref,
        }
    }

    /// Returns code snippets used for declaring field types, initializations,
    /// and references.
    fn gen_field_defs(&self, fields: &[TableSchemaField]) -> FieldSnippets {
        let fields_data = fields
            .iter()
            .map(|f| self.gen_field(f))
            .collect::<Vec<FieldData>>();

        let fields_types = fields_data
            .iter()
            .map(|fd| fd.field_type_decl.as_str())
            .collect::<Vec<&str>>()
            .join(",\n\t");
        let fields_inits = fields_data
            .iter()
            .map(|fd| fd.field_init.as_str())
            .collect::<Vec<&str>>()
            .join(",\n\t");
        let fields_list = fields_data
            .iter()
            .map(|fd| format!("Fields.{0}", fd.field_ref))
            .collect::<Vec<String>>()
            .join(",\n\t");
        FieldSnippets {
            fields_types,
            fields_inits,
            fields_list,
        }
    }
}

impl Generator for Csharp<'_> {
    fn dataset(&self) -> &GetDatasetVersionResponse {
        self.dataset
    }

    fn resource_table(&self, r: &Table) -> DynamicAsset {
        let dataset = self.dataset();
        let dataset_id = dataset.uuid.to_string();
        let dataset_name = self.dataset_name(&dataset.name);
        let namespace = dataset_name.replace(' ', "").to_case(Case::Pascal);

        let resource_name = &r.name;
        let schema = r.schema.as_ref().unwrap();
        let class_name = clean_name(resource_name).to_case(Case::Pascal);
        if let TableSchema::Object { fields, .. } = schema {
            let FieldSnippets {
                fields_inits,
                fields_list,
                fields_types,
            } = self.gen_field_defs(fields);

            #[derive(Serialize)]
            struct Context {
                namespace: String,
                dataset_id: String,
                dataset_name: String,
                dataset_version: String,
                class_name: String,
                resource_name: String,
                fields_types: String,
                fields_inits: String,
                fields_list: String,
            }
            let context = Context {
                namespace,
                dataset_id,
                dataset_name,
                dataset_version: dataset.version.version.to_string(),
                class_name: class_name.clone(),
                resource_name: resource_name.to_string(),
                fields_types,
                fields_inits,
                fields_list,
            };

            let code = match self.tt.render(TABLE_CLASS_TEMPLATE_NAME, &context) {
                Ok(result) => result,
                Err(e) => panic!("Failed to render table class with error {:?}", e),
            };

            let path = Path::new(&self.source_dir())
                .join("Tables")
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

    fn version(&self) -> DynamicAsset {
        let src_dir = self.source_dir();
        let src_dir = Path::new(&src_dir);
        #[derive(Serialize)]
        struct Context {
            code_version: String,
        }
        let context = Context {
            code_version: String::from(CSHARP_VERSION),
        };
        let code = match self.tt.render(VERSION_TEMPLATE_NAME, &context) {
            Ok(result) => result,
            Err(e) => panic!("Failed to render version file with error {:?}", e),
        };

        DynamicAsset {
            path: Box::new(src_dir.join("Version.cs")),
            name: "codeVersion".into(),
            content: code,
        }
    }

    fn static_assets(&self) -> Vec<StaticAsset> {
        Asset::iter()
            .chain(ProtoAsset::iter())
            .map(|p| {
                // Prefix static source paths with this data package's source directory.
                // E.g., `Field.cs` -> `MyDataPackage/Field.cs`.
                let src_dir = self.source_dir();
                let path = Box::new(Path::new(&src_dir).join(p.to_string()));
                let content = Asset::get(&p).unwrap_or_else(|| ProtoAsset::get(&p).unwrap());
                StaticAsset { path, content }
            })
            .collect()
    }

    fn entry_file_name(&self) -> String {
        String::from("")
    }

    fn root_dir(&self) -> PathBuf {
        let dataset = self.dataset();
        let package_directory = format!(
            "{}@{}",
            self.dataset_name(&dataset.name),
            package_version(&dataset.version.version)
        );
        Path::new("csharp").join(package_directory)
    }

    fn source_dir(&self) -> String {
        let dataset = self.dataset();
        let dataset_name = self.dataset_name(&dataset.name);
        dataset_name.to_case(Case::Pascal)
    }

    fn variable_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Pascal)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.cs", name.to_case(Case::Pascal))
    }

    fn dataset_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Pascal)
    }

    fn manifest(&self) -> Manifest {
        let dataset = self.dataset();
        let pkg_name: String = self.dataset_name(&dataset.name);
        let version = package_version(&dataset.version.version);

        let src_dir = self.source_dir();
        let proj_file_name = format!("{pkg_name}.csproj");
        let path = Path::new(&src_dir).join(proj_file_name);
        let contents = format!("
<Project Sdk=\"Microsoft.NET.Sdk\">

  <PropertyGroup>
    <TargetFramework>net6.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <Version>{version}</Version>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include=\"Google.Protobuf\" Version=\"3.24.1\" />
    <PackageReference Include=\"Grpc.Net.Client\" Version=\"2.56.0-pre2\" />
    <PackageReference Include=\"Grpc.Tools\" Version=\"2.57.0\">
      <IncludeAssets>runtime; build; native; contentfiles; analyzers; buildtransitive</IncludeAssets>
        <PrivateAssets>all</PrivateAssets>
    </PackageReference>
    <PackageReference Include=\"Newtonsoft.Json\" Version=\"13.0.3\" />
  </ItemGroup>
  <ItemGroup>
    <Protobuf Include=\"proto\\dpm_agent.proto\" GrpcServices=\"Client\" />
  </ItemGroup>
</Project>");

        Manifest {
            file_name: path.display().to_string(),
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

    /// Builds the generated package. E.g., for the `C#` target, builds the C# package using
    /// the recommended C# build tools: `dotnet build`.
    fn build_package(&self, path: &Path) {
        println!("Building C# package");
        let dataset = self.dataset();
        let pkg_name: String = self.dataset_name(&dataset.name);

        exec_cmd(
            "creating solution file with dotnet",
            path,
            "dotnet",
            &["new", "sln", "--force", "--name", &pkg_name],
        );

        exec_cmd(
            "adding project to solution",
            path,
            "dotnet",
            &["sln", "add", &pkg_name],
        );

        exec_cmd("building with dotnet", path, "dotnet", &["build"]);

        exec_cmd("creating nupkg", path, "dotnet", &["pack"]);
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::{api::DatasetVersion, descriptor::Dataset};

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
        let dataset = Dataset::read("tests/resources/dataset.json").unwrap();
        let res = GetDatasetVersionResponse {
            name: dataset.name.to_string(),
            uuid: Uuid::from_bytes(dataset.id.as_bytes().to_owned()),
            description: dataset.description.unwrap_or("".into()),
            version: DatasetVersion {
                version: dataset.version,
                accelerated: false,
                dataset: dataset.tables,
                patch_state: None,
                patch_state_data: None,
            },
        };
        let generator = Box::new(Csharp::new(&res));
        let expected_dir = format!("TestSnowflake@0.1.0-{}", CSHARP_VERSION);
        assert_eq!(generator.root_dir(), Path::new("csharp").join(expected_dir));
    }
}
