//! Command parsers and logic.

use clap::{Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod snowflake;

use super::codegen::generate_package;
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
    /// TODO(PAT-3374): Connection parameters are discovered automatically using
    /// the same environment variables as those used by SnowSQL. See
    /// https://docs.snowflake.com/en/user-guide/snowsql for details.
    Snowflake {
        /// Table to include in the descriptor
        #[arg(long)]
        table: Vec<String>,

        /// Schema whose tables to include in the descriptor
        #[arg(long)]
        schema: Vec<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Target {
    #[value(name = "typescript")]
    TypeScript,
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

impl App {
    pub async fn exec(self) {
        match self.command {
            Command::Describe { source, output } => {
                match source {
                    DescribeSource::Patch { .. } => {}
                    DescribeSource::Snowflake { table, schema } => {
                        snowflake::describe(table, schema, output).await;
                        ()
                    }
                };
            }
            Command::BuildPackage { source, target } => match read_data_package(&source) {
                Ok(dp) => {
                    for t in target {
                        match t {
                            Target::TypeScript => {
                                println!("Going to build {source} to {:?}", t);
                                generate_package(&dp, &t);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading {source}: {}", e)
                }
            },
        }
    }
}
