use std::fs;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::cli::OutputFormat;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record:Player = result?;
        ret.push(record);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}