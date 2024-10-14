use std::io;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{aot::Bash, generate};

use crate::config;

#[derive(Subcommand, Clone, Debug)]
pub(crate) enum Command {
    Interactive,
    Ping { printer: String },
    Print { printer: String, file: String },
    Upload { printer: String, file: String },
}

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cmd {
    /// What mode to run the program in
    #[command(subcommand)]
    pub mode: Option<Command>,
}

#[derive(Debug)]
pub(crate) struct ArgError {
    _missing_args: Vec<String>,
}
