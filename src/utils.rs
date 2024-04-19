use std::{fmt::Write, fs, io::Read, path::Path};

use anyhow::{Error, Result};

pub fn verify_input(s: &str) -> Result<String> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(anyhow::anyhow!(
            "Invalid input file path, must be a CSV file"
        ))
    }
}

pub fn output_contents(output: &str, contents: &str) {
    if output != "-" {
        fs::write(output, contents).unwrap();
    } else {
        println!("{}", contents);
    }
}

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
    }
}

pub fn get_content(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        Ok(fs::read_to_string(input)?)
    } else {
        Ok(input.to_string())
    }
}

pub fn encode_hex(input: &[u8]) -> String {
    input
        .iter()
        .fold(String::with_capacity(input.len()), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(Into::into))
        .collect::<Result<Vec<u8>, Error>>()
}
