use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize, Serializer};
use crate::to_do::enums::TaskStatus::{DONE, PENDING};

#[derive(Clone, Eq, Debug)]
pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            DONE => "DONE".to_string(),
            PENDING => "PENDING".to_string()
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => DONE,
            "PENDING" => PENDING,
            _ => panic!("input {} not supported", input_string)
        }
    }
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DONE => {
                match other {
                    DONE => true,
                    PENDING => false,
                }
            }
            PENDING => {
                match other {
                    DONE => false,
                    PENDING => true
                }
            }
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            DONE => write!(f, "DONE"),
            PENDING => write!(f, "PENDING")
        }
    }
}
