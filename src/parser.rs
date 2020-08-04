use std::fs::File;
use std::io;
use std::io::BufReader;

use crate::input_data;

pub fn parse_from_json(file: &mut File) -> Result<input_data::InputData, io::Error> {
    let reader = BufReader::new(file);

    let input_data = serde_json::from_reader(reader).expect("Unable to parse json content");

    Ok(input_data)
}