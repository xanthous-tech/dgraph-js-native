use std::collections::HashMap;
use std::string::String;

use serde_json::{Value, Error};

pub struct ResponseWrapper {
  pub json: String,
  pub uids_map: HashMap<String, String>,
}

impl ResponseWrapper {
  pub fn get_json_value(&self) -> Result<Value, Error> {
    serde_json::from_slice(self.json.as_bytes())
  }
}
