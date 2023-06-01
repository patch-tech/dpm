//! Command parsers and logic.

use clap::{Parser, Subcommand};

mod snowflake;

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
    /// environment variables as those used by SnowSQL. See
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
        #[arg(short, long)]
        target: Vec<String>,
    },
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    #[command(subcommand)]
    command: Command,
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

            _ => println!("Subcommand not implemented"),
        }
    }
}
