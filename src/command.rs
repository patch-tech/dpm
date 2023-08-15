//! Command parsers and logic.

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

mod describe;
mod login;
mod publish;
pub mod snowflake;
mod source;
mod update;

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

    /// Build a data package from a data package descriptor
    BuildPackage {
        /// Data package descriptor to read
        #[arg(short, long, value_name = "FILE", default_value = "datapackage.json")]
        descriptor: PathBuf,

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

    Source {
        #[command(subcommand)]
        action: SourceAction,
    },

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
                    eprintln!("describe failed: {:#}", source)
                };
            }
            Command::BuildPackage {
                target,
                descriptor,
                out_dir,
                assume_yes,
            } => match read_data_package(&descriptor) {
                Ok(dp) => {
                    check_output_dir(&out_dir);
                    generate_package(&dp, &target, &out_dir, assume_yes);
                }
                Err(e) => {
                    eprintln!("Error reading {}: {}", descriptor.display(), e)
                }
            },
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
