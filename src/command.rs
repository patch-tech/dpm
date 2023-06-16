//! Command parsers and logic.

use clap::{Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod snowflake;

use super::codegen::generate_package;
use super::codegen::Generator;
use super::codegen::Python;
use super::codegen::TypeScript;
use super::descriptor::DataPackage;

#[derive(Subcommand)]
enum DescribeSource {
    /// Describe data in Patch
    Patch {
        /// Name of dataset to describe
        dataset: String,
    },

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
    #[value(name = "typescript")]
    TypeScript,
    #[value(name = "python")]
    Python,
}

impl Target {
    pub fn generator_for_package<'a>(&self, dp: &'a DataPackage) -> Box<dyn Generator + 'a> {
        let generator: Box<dyn Generator> = match self {
            Target::TypeScript => Box::new(TypeScript::new(dp)),
            Target::Python => Box::new(Python::new(dp)),
        };
        generator
    }
}

#[derive(Subcommand)]
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
    },
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

/// Reads datapackage.json at path and returns a deserialized instance of DataPackage.
/// Modified from example code at: https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html#example
fn read_data_package<P: AsRef<Path>>(path: P) -> Result<DataPackage, Box<dyn Error>> {
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

impl App {
    pub async fn exec(self) {
        match self.command {
            Command::Describe {
                source,
                output,
            } => {
                match source {
                    DescribeSource::Patch { .. } => {}
                    DescribeSource::Snowflake { name, table, schema } => {
                        let package = snowflake::describe(name, table, schema, output).await;
                        println!("{}", serde_json::to_string_pretty(&package).unwrap());
                    }
                };
            }
            Command::BuildPackage {
                source,
                target,
                output,
            } => match read_data_package(&source) {
                Ok(dp) => {
                    let output = Path::new(&output);
                    check_output_dir(&output);

                    for t in target {
                        generate_package(&dp, &t, output);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading {source}: {}", e)
                }
            },
        }
    }
}
