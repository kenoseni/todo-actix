// used for file operations
use std::fs;
use std::fs::File;

// used for reading file contents
use std::io::Read;

// json macro, used for creating JSON values
use serde_json::json;

// used for representing JSON data
use serde_json::value::Value;
use serde_json::Map;

/// The given Rust code is a function that reads a file containing JSON data, converts it into a JSON object, and returns it as a Map<String, Value>. Let's break down the code and understand each part:
pub fn read_file(file_name: &str) -> Map<String, Value> {
    // get file data

    // It opens the file specified by file_name using File::open. The unwrap method is called to handle any errors that may occur during file opening. If the file cannot be opened, the program will panic.
    let mut file = File::open(file_name.to_string()).unwrap();

    let mut data = String::new();

    // Add file data as a string to the data variable
    // The contents of the file are read and stored as a string in the data variable using the read_to_string method of the file object. Again, unwrap is called to handle any errors that may occur during file reading.
    file.read_to_string(&mut data).unwrap();

    // convert file data in data variable to json from string

    // The JSON data in the data variable is converted into a JSON value using serde_json::from_str. This method parses the string and returns a Value representing the JSON structure. If the parsing fails, unwrap will handle the error and panic.
    let json: Value = serde_json::from_str(&data).unwrap();

    // package data in Map

    // The as_object method is called on the json value to convert it into a reference to a JSON object. The unwrap method is used to handle the case when the JSON value is not an object. The clone method is then called to create a copy of the JSON object and store it in the state variable.
    let state: Map<String, Value> = json.as_object().unwrap().clone();

    return state;
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);

    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write to file");
}
