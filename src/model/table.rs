use std::path::PathBuf;

use eyre::{eyre, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TableName(String);

impl TableName {
    pub fn from(name: &str) -> Result<Self> {
        if name.is_empty() {
            Err(eyre!("Table name cannot be empty"))
        } else {
            Ok(Self(name.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn new() -> Result<Self> {
        Ok(Self {
            headers: Vec::new(),
            rows: Vec::new(),
        })
    }

    pub fn from_csv(_file_path: PathBuf) -> Result<Self> {
        todo!()
    }
}
