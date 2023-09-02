//! Csharp code generator.

use super::generator::{exec_cmd, DynamicAsset, Generator, ItemRef, Manifest, StaticAsset};
use crate::api::GetPackageVersionResponse;
use crate::descriptor::{DataResource, TableSchema, TableSchemaField};
use convert_case::{Case, Casing};
use regress::Regex;
use rust_embed::RustEmbed;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tinytemplate::TinyTemplate;

pub struct Csharp<'a> {
    pub data_package: &'a GetPackageVersionResponse,
    tt: TinyTemplate<'a>,
}

const CSHARP_VERSION: &str = "0.1.0";

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
    code: String,
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
static FIELD_DEF_TEMPLATE: &str = "[\"{field_ref}\"] = new {field_type}(\"{field_name}\")";

static TABLE_CLASS_TEMPLATE_NAME: &str = "table";
static TABLE_CLASS_TEMPLATE: &str = "
using dpm;

namespace {namespace} \\{
  public class {class_name} \\{
    // Fields.
    public static Dictionary<string, FieldExpr> Fields = new  Dictionary<string, FieldExpr>\\{
        {field_defs}
    };

    private Table table_;

    // Singleton.
    private static readonly Lazy<{class_name}> lazy = new Lazy<{class_name}>(() => new {class_name}());

    private {class_name}() \\{
      this.table_ = new Table(
        packageId: \"{package_id}\",
        datasetName: \"{dataset_name}\",
        datasetVersion: \"{dataset_version}\",
        name: \"{resource_name}\",
        fields: {class_name}.Fields.Values.ToArray()
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
namespace dpm \\{
    public static class Constants \\{
        public const string CODE_VERSION = \"{code_version}\";
    }
}
";

impl<'a> Csharp<'a> {
    pub fn new(dp: &'a GetPackageVersionResponse) -> Self {
        let mut tt = TinyTemplate::new();
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
            .add_template(VERSION_TEMPLATE_NAME, VERSION_TEMPLATE)
            .is_err()
        {
            panic!("Failed to add {:?} template", VERSION_TEMPLATE_NAME);
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
        let (field_name, field_type) = match field {
            TableSchemaField::NumberField { name, .. } => {
                (name.to_string(), String::from("Field<float>"))
            }
            TableSchemaField::IntegerField { name, .. } => {
                (name.to_string(), String::from("Field<int>"))
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

        FieldData { code }
    }

    /// Returns a code snippet declaring the fields map.
    fn gen_field_defs(&self, fields: &[TableSchemaField]) -> String {
        let fields_data = fields
            .iter()
            .map(|f| self.gen_field(f))
            .collect::<Vec<FieldData>>();
        fields_data
            .iter()
            .map(|fd| fd.code.as_str())
            .collect::<Vec<&str>>()
            .join(",\n\t")
    }
}

impl Generator for Csharp<'_> {
    fn data_package(&self) -> &GetPackageVersionResponse {
        self.data_package
    }

    fn resource_table(&self, r: &DataResource) -> DynamicAsset {
        let dp = self.data_package();
        let package_id = format!("{}", dp.package_uuid);
        let dataset_name = self.package_name(&dp.package_name);
        let namespace = dataset_name.replace(' ', "").to_case(Case::Pascal);

        let resource_name = &r.name;
        let schema = r.schema.as_ref().unwrap();
        let class_name = clean_name(resource_name).to_case(Case::Pascal);
        if let TableSchema::Object { fields, .. } = schema {
            let field_defs = self.gen_field_defs(fields);

            #[derive(Serialize)]
            struct Context {
                namespace: String,
                package_id: String,
                dataset_name: String,
                dataset_version: String,
                class_name: String,
                resource_name: String,
                field_defs: String,
            }
            let context = Context {
                namespace,
                package_id,
                dataset_name,
                dataset_version: dp.version.version.to_string(),
                class_name: class_name.clone(),
                resource_name: resource_name.to_string(),
                field_defs,
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
        let dp = self.data_package();
        let package_directory = format!(
            "{}@{}.{}",
            self.package_name(&dp.package_name),
            dp.version.version,
            CSHARP_VERSION
        );
        Path::new("csharp").join(package_directory)
    }

    fn source_dir(&self) -> String {
        let dp = self.data_package();
        let dataset_name = self.package_name(&dp.package_name);
        dataset_name.to_case(Case::Pascal)
    }

    fn variable_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Camel)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.cs", name.to_case(Case::Pascal))
    }

    fn package_name(&self, name: &str) -> String {
        clean_name(name).to_case(Case::Pascal)
    }

    fn manifest(&self) -> Manifest {
        let dp = self.data_package();
        let pkg_name: String = self.package_name(&dp.package_name);
        let version = format!("{}-{}", dp.version.version, CSHARP_VERSION);

        let src_dir = self.source_dir();
        let proj_file_name = format!("{pkg_name}.csproj");
        let path = Path::new(&src_dir).join(proj_file_name);
        let contents = format!("
<Project Sdk=\"Microsoft.NET.Sdk\">

  <PropertyGroup>
    <TargetFramework>net6.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <ReleaseVersion>{version}</ReleaseVersion>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include=\"Google.Protobuf\" Version=\"3.24.1\" />
    <PackageReference Include=\"Grpc.Net.Client\" Version=\"2.56.0-pre2\" />
    <PackageReference Include=\"Grpc.Tools\" Version=\"2.57.0\">
      <IncludeAssets>runtime; build; native; contentfiles; analyzers; buildtransitive</IncludeAssets>
        <PrivateAssets>all</PrivateAssets>
    </PackageReference>
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
        let dp = self.data_package();
        let pkg_name: String = self.package_name(&dp.package_name);

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

        exec_cmd(
            "building with dotnet",
            path,
            "dotnet",
            &["build"], // TODO: complete args.
        );
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::{api::PackageVersion, command::read_data_package};

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
        let dp = read_data_package("tests/resources/datapackage.json").unwrap();
        let res = GetPackageVersionResponse {
            package_name: dp.name.to_string(),
            package_uuid: Uuid::from_bytes(dp.id.as_bytes().to_owned()),
            package_description: dp.description.unwrap_or("".into()),
            version: PackageVersion {
                version: dp.version,
                dataset: dp.dataset,
            },
        };
        let generator = Box::new(Csharp::new(&res));
        let expected_dir = format!("TestSnowflake@0.1.0.{}", CSHARP_VERSION);
        assert_eq!(generator.root_dir(), Path::new("csharp").join(expected_dir));
    }
}
