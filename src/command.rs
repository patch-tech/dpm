//! Command parsers and logic.

use clap::{CommandFactory, Parser, Subcommand};
use std::error::Error;
use std::fs::{write, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

mod snowflake;

use super::codegen::{generate_package, Target};
use super::descriptor::DataPackage;
use clap_complete::{self, generate, Shell};

#[derive(Subcommand, Debug)]
enum DescribeSource {
    /// Describe data in Snowflake
    ///
    /// Connection parameters are discovered automatically using the same
    /// environment variables as those used by SnowSQL:
    ///
    /// - SNOWSQL_ACCOUNT ({org_name}-{account_name})
    /// - SNOWSQL_USER
    /// - SNOWSQL_PWD
    /// - SNOWSQL_DATABASE
    /// - SNOWSQL_SCHEMA
    ///
    /// See https://docs.snowflake.com/en/user-guide/snowsql-start for details.
    ///
    /// If no optional arguments are given, all tables in the database given by
    /// `SNOWSQL_DATABASE` are included in the descriptor.
    #[clap(verbatim_doc_comment)]
    Snowflake {
        /// `name` to set in the descriptor.
        #[arg(short, long)]
        name: String,

        /// Table to include in the descriptor. May be given multiple times.
        #[arg(long)]
        table: Vec<String>,

        /// Schema whose tables to include in the descriptor. May be given multiple times.
        #[arg(long)]
        schema: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a data package descriptor that describes some source data
    Describe {
        /// Path to write descriptor to.
        #[arg(short, long, default_value = "datapackage.json")]
        output: PathBuf,

        #[command(subcommand)]
        source: DescribeSource,
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
pub fn read_data_package<P: AsRef<Path>>(path: P) -> Result<DataPackage, Box<dyn Error>> {
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
            Command::Describe { source, ref output } => {
                let package: DataPackage = match source {
                    DescribeSource::Snowflake {
                        name,
                        table,
                        schema,
                    } => snowflake::describe(name, table, schema).await,
                };

                if package.resources.is_empty() {
                    panic!("No resources found. Please check your table and schemas names.")
                }
                match write(output, serde_json::to_string_pretty(&package).unwrap()) {
                    Ok(()) => eprintln!("wrote descriptor: {}", output.display()),
                    Err(e) => eprintln!("error while writing descriptor: {}", e),
                }
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
                    eprintln!("Error reading {}: {}", descriptor.to_string_lossy(), e)
                }
            },
            Command::Update { descriptor } => match read_data_package(&descriptor) {
                Ok(_dp) => eprintln!("found {}", descriptor.display()),
                Err(e) => {
                    eprintln!("Error reading {}: {}", descriptor.to_string_lossy(), e)
                }
            },
            Command::Completions { shell } => {
                let mut cmd = App::command();
                print_completions(shell, &mut cmd);
            }
        }
    }
}
