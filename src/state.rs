use std::fs;
use std::fs::File;
use std::io::Read;

// json macro
use serde_json::json;

use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    // get file data
    let mut file = File::open(file_name.to_string()).unwrap();

    let mut data = String::new();
    // Add file data as a string to the data variable
    file.read_to_string(&mut data).unwrap();

    // convert file data in data variable to json from string
    let json: Value = serde_json::from_str(&data).unwrap();

    // package data in Map
    let state: Map<String, Value> = json.as_object().unwrap().clone();

    return state;
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);

    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write to file");
}
