// File: opts.rs
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "csv file to json file conversion.")]
    Csv(CsvOpts),
}
//
#[derive(Debug, Parser)]
#[clap(name = "rcil", version, author, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub(crate) cmd: Subcommand,
}

#[derive(Debug,Parser)]
pub struct CsvOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}