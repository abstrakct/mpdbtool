use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Database commands
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    /// XML commands,
    Xml {
        #[command(subcommand)]
        command: XmlCommands,
    },
}

#[derive(Subcommand)]
pub enum DbCommands {
    /// Populate the database from a local XML file
    Populate,
    /// Reset the database (delete all data) (not implemented yet)
    Reset,
}

#[derive(Subcommand)]
pub enum XmlCommands {
    /// Convert xml to yml
    Convert,
}
