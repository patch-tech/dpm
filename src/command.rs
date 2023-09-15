//! Command parsers and logic.

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use semver::Version;
use std::fs::{create_dir_all, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

mod describe;
mod login;
mod publish;
pub mod snowflake;
mod source;
mod update;
use crate::api::{GetPackageVersionResponse, PackageVersion};
use crate::{api::Client, session};

use self::source::SourceAction;
use super::codegen::{generate_package, Target};
use super::descriptor::{DataPackage, Name};
use clap_complete::{self, generate, Shell};

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a data package descriptor that describes some source's data.
    Describe {
        /// Name of source to describe.
        source_name: String,

        /// Path to write descriptor to.
        #[arg(short, long, value_name = "PATH", default_value = "datapackage.json")]
        output: PathBuf,

        /// Display name to give the data package that will be created from the
        /// resulting descriptor.
        #[arg(short, long)]
        package_name: Name,

        /// Additional, source-type-specific refinements to apply while
        /// describing the source.
        #[command(subcommand)]
        refinement: Option<describe::DescribeRefinement>,
    },

    /// Build an instance of a data package.
    ///
    /// There are two different ways to specify the package to build:
    ///
    /// 1. By default (or with -d/--descriptor <FILE>) a package descriptor file
    ///    on the filesystem is used to define the tables and fields accessible
    ///    by the package.
    /// 2. With -p/--package <PACKAGE_REF>, an instance of the referenced,
    ///    published package is built.
    ///
    /// A package created via (1) is called a draft data package. It is only
    /// usable by the DPM Cloud user that created it; queries made by any other
    /// principal will not be authorized.
    ///
    /// A package created via (2) is called a standard data package. Queries
    /// made using it will be authorized if and only if the package's
    /// authorization policy in DPM Cloud allows querying by the given
    /// principal.
    #[command(verbatim_doc_comment)]
    BuildPackage {
        /// Data package descriptor to read.
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        descriptor: PathBuf,

        /// Data package identifier: "<package name>@<version>".
        /// Conflicts with --descriptor.
        #[arg(short, long, value_name = "PACKAGE_REF", conflicts_with = "descriptor")]
        package: Option<String>,

        /// Directory to write build artifacts to.
        #[arg(short, long, value_name = "DIR", default_value = "dist")]
        out_dir: PathBuf,

        /// Automatically respond "yes" to any prompts.
        #[arg(name = "yes", short, long)]
        assume_yes: bool,

        /// Type of data package to build
        #[command(subcommand)]
        target: Target,
    },

    /// Log into DPM Cloud.
    Login,

    /// Create a data package in DPM Cloud.
    Publish {
        /// Data package descriptor to read
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        descriptor: PathBuf,
    },

    /// Create and list data sources
    Source {
        #[command(subcommand)]
        action: SourceAction,
    },

    /// Update a data package to a new version
    Update {
        /// Data package descriptor to update
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        descriptor: PathBuf,
    },

    /// Write the tab completion file for a shell
    Completions {
        /// Shell to generate completion file for
        shell: Shell,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

/// Reads datapackage.json at path and returns a deserialized instance of DataPackage.
/// Modified from example code at: https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html#example
pub fn read_data_package<P: AsRef<Path>>(path: P) -> Result<DataPackage> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let data_package = serde_json::from_reader(reader)?;
    Ok(data_package)
}

/// Checks that the output directory exists and is accessible.
fn check_output_dir(p: &Path) {
    match p.try_exists() {
        Ok(v) if !v => panic!("Output directory {:?} does not exist", p),
        Err(e) => {
            panic!("Error accessing output directory {e}")
        }
        _ => {}
    }
}

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

impl App {
    pub async fn exec(self) {
        match self.command {
            Command::Describe {
                source_name,
                package_name,
                output,
                refinement,
            } => {
                if let Err(source) =
                    describe::describe(&source_name, &package_name, &output, refinement.as_ref())
                        .await
                {
                    eprintln!("describe failed: {:#}", source);
                    std::process::exit(1);
                };
            }
            Command::BuildPackage {
                descriptor,
                package,
                target,
                out_dir,
                assume_yes,
            } => {
                // `descriptor` is always defined (possibly via its
                // default_value), whereas the caller may instead opt to build a
                // published package via -p. Before reaching this function, clap
                // will have verified that if -p was given, -d was not given.
                let build_input: GetPackageVersionResponse = if let Some(package_ref) = package {
                    let package_identifier: Vec<&str> = package_ref.split('@').collect();
                    let version: Version = Version::parse(package_identifier[1])
                        .expect("package identifier `version` is invalid");
                    let session = session::get_token().expect("unable to get session");
                    let client = Client::new(&session).expect("unable to get client");

                    client
                        .get_package_version(package_identifier[0], version)
                        .await
                        .expect("failed to fetch package")
                } else {
                    let dp = match read_data_package(&descriptor) {
                        Ok(dp) => dp,
                        Err(e) => panic!("Error reading {}: {}", descriptor.display(), e),
                    };
                    GetPackageVersionResponse {
                        package_name: dp.name.to_string(),
                        package_uuid: uuid::Uuid::parse_str(&dp.id.to_string()).unwrap(),
                        package_description: dp.description.unwrap_or("".into()),
                        version: PackageVersion {
                            version: dp.version,
                            dataset: dp.dataset,
                        },
                    }
                };

                create_dir_all(&out_dir).expect("error creating output directory");
                check_output_dir(&out_dir);
                generate_package(&build_input, &target, &out_dir, assume_yes);
            }
            Command::Login => {
                if let Err(source) = login::login().await {
                    eprintln!("login failed: {}", source)
                };
            }
            Command::Publish { descriptor } => match publish::publish(&descriptor).await {
                Ok(_) => (),
                Err(e) => eprintln!("publish failed: {}", e),
            },
            Command::Source {
                action: SourceAction::Create(cs),
            } => match source::create(&cs).await {
                Ok(()) => (),
                Err(e) => eprintln!("error creating source: {}", e),
            },
            Command::Source {
                action: SourceAction::List,
            } => match source::list().await {
                Ok(()) => (),
                Err(e) => eprintln!("error listing sources: {}", e),
            },
            Command::Update { descriptor } => {
                match update::update(&descriptor).await {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("error: {:#}", e);
                        // e.chain()
                        //     .skip(1)
                        //     .for_each(|cause| eprintln!("  ...because: {}", cause));
                    }
                };
            }
            Command::Completions { shell } => {
                let mut cmd = App::command();
                print_completions(shell, &mut cmd);
            }
        }
    }
}
