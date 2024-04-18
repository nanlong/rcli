use csv::{Reader, ReaderBuilder, StringRecord};
use serde::Serialize;
use serde_json::Value;
use std::fs::{self, File};

use crate::OutputFormat;

#[derive(Debug)]
struct CsvRecord {
    headers: Option<StringRecord>,
    records: Vec<StringRecord>,
}

impl CsvRecord {
    fn new(headers: Option<StringRecord>, records: Vec<StringRecord>) -> Self {
        Self { headers, records }
    }

    fn to_raw(&self) -> String {
        let mut ret = vec![];
        if let Some(headers) = &self.headers {
            ret.push(headers.clone());
        }
        ret.extend(self.records.clone());
        ret.iter()
            .map(|record| record.iter().collect::<Vec<_>>().join(","))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn to_value(&self) -> Vec<Value> {
        let mut ret = vec![];
        if let Some(headers) = &self.headers {
            for record in &self.records {
                let record = headers.iter().zip(record.iter()).collect::<Value>();
                ret.push(record);
            }
        } else {
            for record in &self.records {
                let record = record.iter().collect::<Value>();
                ret.push(record);
            }
        }
        ret
    }
}

impl TryFrom<Reader<File>> for CsvRecord {
    type Error = anyhow::Error;

    fn try_from(mut rdr: Reader<File>) -> Result<Self, Self::Error> {
        let headers = rdr.headers()?.clone();
        let records = rdr.records().collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            headers: Some(headers),
            records,
        })
    }
}

#[derive(Debug, Serialize)]
struct Toml {
    items: Vec<Value>,
}

impl Toml {
    fn new(items: Vec<Value>) -> Self {
        Self { items }
    }
}

fn read_csv(input: &str, delimiter: char, no_header: bool) -> anyhow::Result<CsvRecord> {
    let rdr = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_path(input)?;

    let csv_record: CsvRecord = rdr.try_into()?;

    if !no_header {
        Ok(csv_record)
    } else {
        Ok(CsvRecord::new(None, csv_record.records))
    }
}

fn csv_convert(csv_record: CsvRecord, format: OutputFormat) -> anyhow::Result<String> {
    match format {
        OutputFormat::Raw => Ok(csv_record.to_raw()),
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&csv_record.to_value())?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&csv_record.to_value())?),
        OutputFormat::Toml => Ok(toml::to_string(&Toml::new(csv_record.to_value()))?),
    }
}

fn output_contents(output: &str, contents: &str) {
    if output != "-" {
        fs::write(output, contents).unwrap();
    } else {
        println!("{}", contents);
    }
}

pub fn process_csv(
    input: &str,
    output: &str,
    format: OutputFormat,
    delimiter: char,
    no_header: bool,
) -> anyhow::Result<()> {
    let csv_record: CsvRecord = read_csv(input, delimiter, no_header)?;
    let contents = csv_convert(csv_record, format)?;
    output_contents(output, &contents);
    Ok(())
}
