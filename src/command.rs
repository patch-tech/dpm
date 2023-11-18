//! Command parsers and logic.

use clap::{CommandFactory, Parser, Subcommand};

use std::io;
use std::path::PathBuf;

mod build_package;
mod init;
mod login;
mod package;
mod publish;
pub mod snowflake;
mod source;
mod update;

use self::package::DatasetAction;
use self::source::SourceAction;
use super::codegen::Target;
use super::descriptor::Name;
use clap_complete::{self, generate, Shell};

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a descriptor that specifies a dataset containing data from some
    /// source.
    Init {
        /// Name of source that will supply data for the dataset.
        #[arg(short, long = "source", value_name = "NAME")]
        source_name: String,

        /// Name to give the dataset specified by the descriptor.
        #[arg(short, long = "name", value_name = "NAME")]
        dataset_name: Name,

        /// Path to write descriptor to.
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        output: PathBuf,

        /// Additional, source-type-specific filters to apply while performing
        /// catalog discovery on the source.
        #[command(subcommand)]
        refinement: Option<init::DescribeRefinement>,
    },

    /// Build a data package: a code library to query a specific dataset
    ///
    /// There are two ways to specify the package to build:
    ///
    /// 1. By default (or with -d/--descriptor <FILE>) a dataset descriptor on
    ///    the filesystem is used to define the tables and fields accessible by
    ///    the package.
    /// 2. With -p/--package <PACKAGE_REF>, an instance of the referenced,
    ///    published dataset is built.
    ///
    /// A package created via (1) is called a draft data package. It is only
    /// usable by the Patch user that created it; queries made by any other
    /// principal will not be authorized.
    ///
    /// A package created via (2) is called a release data package. Queries made
    /// using it will be authorized if and only if the package's authorization
    /// policy in Patch allows querying by the given principal.
    #[command(verbatim_doc_comment)]
    BuildPackage {
        /// Dataset descriptor to read.
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        descriptor: PathBuf,

        /// Dataset identifier of the form "<package name>@<version>".
        /// Conflicts with --descriptor.
        #[arg(short, long, value_name = "DATASET_REF", conflicts_with = "descriptor")]
        package: Option<String>,

        /// Directory to write build artifacts to.
        #[arg(short, long, value_name = "DIR", default_value = "dist")]
        out_dir: PathBuf,

        /// Automatically respond "yes" to any prompts.
        #[arg(name = "yes", short, long)]
        assume_yes: bool,

        /// Type of data package to build.
        #[command(subcommand)]
        target: Target,
    },

    /// Log into the CLI by authenticating with Patch
    Login,

    /// Interact with datasets
    Dataset {
        #[command(subcommand)]
        action: DatasetAction,
    },

    /// Publish a dataset to Patch
    Publish {
        /// Data package descriptor to read
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        descriptor: PathBuf,
    },

    /// Create and list datasets
    Source {
        #[command(subcommand)]
        action: SourceAction,
    },

    /// Update (refresh) the tables in a dataset
    ///
    /// During an update the tables in the input descriptor are compared to
    /// their counterparts in sources. A summary of the differences is printed
    /// and the user is prompted to accept or reject them all. If they accept,
    /// the input descriptor is copied to a new file with the ".backup" suffix
    /// appended to its file name. In its place an updated descriptor is written
    /// that specifies a dataset containing the same set of tables, but with
    /// up-to-date schemas.
    ///
    /// After this operation one will typically repeat `build-package` to
    /// validate the resulting dataset, and once they're satisfied will run
    /// `publish` to make the new version available to others.
    Update {
        /// Dataset descriptor to update
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

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

impl App {
    pub async fn exec(self) {
        match self.command {
            Command::Init {
                source_name,
                dataset_name: package_name,
                output,
                refinement,
            } => {
                if let Err(source) =
                    init::init(&source_name, &package_name, &output, refinement).await
                {
                    eprintln!("init failed: {:#}", source);
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
                if let Err(e) =
                    build_package::build(descriptor, package, target, out_dir, assume_yes).await
                {
                    eprintln!("package build failed: {:#}", e);
                    std::process::exit(1);
                }
            }
            Command::Login => {
                if let Err(source) = login::login().await {
                    eprintln!("login failed: {}", source)
                };
            }
            Command::Dataset {
                action: DatasetAction::List,
            } => {
                if let Err(e) = package::list().await {
                    eprintln!("dataset listing failed: {:#}", e);
                    std::process::exit(1);
                }
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
