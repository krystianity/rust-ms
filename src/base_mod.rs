pub mod msbase {

    pub extern crate serde_json;
    use std::fs::File;
    use std::io::prelude::*;
    pub use self::serde_json::{Value};

    pub fn read_file(filename: &'static str) -> Option<String> {
        let mut file_handle = File::open(filename).expect("file not found");
        let mut content = String::new();
        match file_handle.read_to_string(&mut content) {
            Ok(_) => Some(content),
            Err(_) => None
        }
    }

    pub fn parse_json(json: String) -> Option<Value> {
        let c_str: &str = &json;
        let parsed = serde_json::from_str(c_str).unwrap();
        match parsed {
            Some(value) => Some(value),
            None => None
        }
    }

    pub mod config {

        extern crate serde_json;
        use super::serde_json::{Value};

        pub fn get_config(filename: Option<&'static str>) -> Option<Value> {
            let content: String = super::read_file(
                filename.unwrap_or("./config/default.json"))
                .unwrap_or(String::from("{}"));
            super::parse_json(content)
        }
    }
}