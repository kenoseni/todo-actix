use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING,
}

// Calling this will enable us to print out the status of the to-do task in the console and write it in our JSON file.
// if we want to write to a file or database, we are going to have to build a method to enable our enum to be represented in a string format.
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}

// Calling this will enable us to print out the status of the to-do task in the console and write it in our JSON file.
// if we want to write to a file or database, we are going to have to build a method to enable our enum to be represented in a string format.
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }
            &Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}
