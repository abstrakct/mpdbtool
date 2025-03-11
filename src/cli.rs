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
    #[command(arg_required_else_help = true)]
    Populate {
        #[arg(
            short = 'x',
            long = "xml",
            group = "format",
            help = "Input data is in XML format",
            conflicts_with = "yml"
        )]
        xml: bool,

        #[arg(
            short = 'y',
            long = "yml",
            group = "format",
            help = "Input data is in YAML format",
            conflicts_with = "xml"
        )]
        yml: bool,
    },
    /// Reset the database (delete all data) (not implemented yet)
    Reset,
}

#[derive(Subcommand)]
pub enum XmlCommands {
    /// Convert xml to yml
    Convert,
}
