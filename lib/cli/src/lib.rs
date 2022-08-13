use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("Args parsing failed")]
    ParseError,
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Sets a custom config file
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FILE",
        default_value = "./config.yaml"
    )]
    pub config_path: PathBuf,
}

pub struct CLI {}

impl CLI {
    pub fn parse_args() -> Result<Args, CLIError> {
        // TODO: Add here the workflow to add a new user (as admin)
        Ok(Args::parse())
        //   Err(CLIError::ParseError)
    }
}
