mod csv;
mod pass;

use clap::Parser;
use csv::CsvOpts;
use pass::GenPassOpts;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "csv file to json file conversion.")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate a random password.")]
    GenPass(GenPassOpts),
}
//
#[derive(Debug, Parser)]
#[clap(name = "rcil", version, author, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub(crate) cmd: Subcommand,
}
