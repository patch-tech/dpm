//! Command parsers and logic.

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

mod snowflake;

use super::codegen::generate_package;
use super::codegen::Generator;
use super::codegen::NodeJs;
use super::codegen::Python;
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

#[derive(ValueEnum, Clone, Debug)]
pub enum Target {
    #[value(name = "nodejs")]
    NodeJs,
    #[value(name = "python")]
    Python,
}

impl Target {
    pub fn generator_for_package<'a>(&self, dp: &'a DataPackage) -> Box<dyn Generator + 'a> {
        let generator: Box<dyn Generator> = match self {
            Target::NodeJs => Box::new(NodeJs::new(dp)),
            Target::Python => Box::new(Python::new(dp)),
        };
        generator
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a data package descriptor that describes some source data
    Describe {
        /// Path to write descriptor to, `-` for stdout
        #[arg(short, long)]
        output: Option<String>,

        #[command(subcommand)]
        source: DescribeSource,
    },

    /// Build data packages from a data package descriptor
    BuildPackage {
        /// Either a file (or `-`), npm:// URL, or pip:// URL
        source: String,

        /// Packages to build
        #[arg(short, long, value_enum)]
        target: Vec<Target>,

        /// Output directory path (must exist)
        #[arg(short, long)]
        output: String,

        #[arg(short = 'y', long)]
        assume_yes: bool,
    },

    /// Write completion file for shell
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
            Command::Describe { source, output } => {
                match source {
                    DescribeSource::Snowflake {
                        name,
                        table,
                        schema,
                    } => {
                        let package = snowflake::describe(name, table, schema, output).await;
                        println!("{}", serde_json::to_string_pretty(&package).unwrap());
                    }
                };
            }
            Command::BuildPackage {
                source,
                target,
                output,
                assume_yes,
            } => match read_data_package(&source) {
                Ok(dp) => {
                    let output = Path::new(&output);
                    check_output_dir(output);

                    for t in target {
                        generate_package(&dp, &t, output, assume_yes);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading {source}: {}", e)
                }
            },
            Command::Completions { shell } => {
                let mut cmd = App::command();
                print_completions(shell, &mut cmd);
            }
        }
    }
}
