use std::{io::Read, path::Path};

pub fn verify_input(s: &str) -> anyhow::Result<String> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(anyhow::anyhow!(
            "Invalid input file path, must be a CSV file"
        ))
    }
}

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
    }
}
