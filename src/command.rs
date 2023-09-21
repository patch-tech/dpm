//! Command parsers and logic.

use clap::{CommandFactory, Parser, Subcommand};

use std::io;
use std::path::PathBuf;

mod build_package;
mod describe;
mod login;
mod publish;
pub mod snowflake;
mod source;
mod update;

use self::source::SourceAction;
use super::codegen::Target;
use super::descriptor::Name;
use clap_complete::{self, generate, Shell};

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a data package descriptor that describes some source's data.
    Init {
        /// Name of source that will supply data for the data package.
        source_name: String,

        /// Path to write descriptor to.
        #[arg(short, long, value_name = "PATH", default_value = "datapackage.json")]
        output: PathBuf,

        /// Display name to give the data package that will be created from the
        /// resulting descriptor.
        #[arg(short, long)]
        package_name: Name,

        /// Additional, source-type-specific refinements to apply while
        /// introspecting the source.
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
    /// A package created via (2) is called a release data package. Queries
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

    /// Update (refresh) the table definitions in a data package.
    ///
    /// During an update the tables in the input descriptor are introspected
    /// anew. A summary of the differences is printed and the user is prompted
    /// to accept or reject them all. If they accept, the input descriptor is
    /// copied to a new file with the ".backup" suffix appended to its file
    /// name, and in its place an updated descriptor is written, reflecting the
    /// current contents of the tables in the package's source.
    ///
    /// After this operation one will typically repeat `build-package` to
    /// validate the resulting data package, and once they're satisfied will run
    /// `publish` to release the new version.
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

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

impl App {
    pub async fn exec(self) {
        match self.command {
            Command::Init {
                source_name,
                package_name,
                output,
                refinement,
            } => {
                if let Err(source) =
                    describe::init(&source_name, &package_name, &output, refinement.as_ref()).await
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
