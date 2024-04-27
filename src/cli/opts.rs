use std::fmt;
use std::str::FromStr;
// File: opts.rs
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
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
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(long, value_parser=parse_format,default_value = "Json")]
    pub format: OutputFormat,
}

pub fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for  &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format. Please use Json or Yaml.")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

