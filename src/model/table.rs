use std::path::PathBuf;

use eyre::{bail, eyre, Result};

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

#[derive(Debug)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn new() -> Result<Self> {
        Ok(Self {
            headers: vec!["".to_string()],
            rows: vec![vec!["".to_string()]],
        })
    }

    pub fn from_csv(_file_path: PathBuf) -> Result<Self> {
        todo!()
    }

    pub fn expand_row(&mut self) -> Result<()> {
        self.rows.push(vec!["".to_string(); self.headers.len()]);
        Ok(())
    }

    pub fn collapse_row(&mut self) -> Result<()> {
        if self.rows.len() > 1 {
            self.rows.pop();
        }
        Ok(())
    }

    pub fn expand_col(&mut self) -> Result<()> {
        self.headers.push("".to_string());
        for row in self.rows.iter_mut() {
            row.push("".to_string());
        }
        Ok(())
    }

    pub fn collapse_col(&mut self) -> Result<()> {
        if self.headers.len() > 1 {
            self.headers.pop();
            for row in self.rows.iter_mut() {
                row.pop();
            }
        }
        Ok(())
    }

    pub fn update_cell(&mut self, row: usize, col: usize, value: &str) -> Result<()> {
        if row >= self.rows.len() || col >= self.headers.len() {
            bail!("Invalid row or column index")
        }

        self.rows[row][col] = value.to_string();
        Ok(())
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Result<&str> {
        if row >= self.rows.len() || col >= self.headers.len() {
            bail!("Invalid row or column index")
        }

        Ok(&self.rows[row][col])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_name() {
        let table_name = TableName::from("test_table").unwrap();
        assert_eq!(table_name.as_str(), "test_table");

        let table_name_err = TableName::from("").unwrap_err();
        assert_eq!(table_name_err.to_string(), "Table name cannot be empty");
    }

    #[test]
    fn test_table_data() {
        let mut table_data = TableData::new().unwrap();
        assert_eq!(table_data.headers.len(), 1);
        assert_eq!(table_data.rows.len(), 1);

        table_data.expand_row().unwrap();
        assert_eq!(table_data.rows.len(), 2);

        table_data.collapse_row().unwrap();
        assert_eq!(table_data.rows.len(), 1);

        table_data.expand_col().unwrap();
        assert_eq!(table_data.headers.len(), 2);
        assert_eq!(table_data.rows[0].len(), 2);

        table_data.collapse_col().unwrap();
        assert_eq!(table_data.headers.len(), 1);
        assert_eq!(table_data.rows[0].len(), 1);

        table_data.update_cell(0, 0, "test").unwrap();
        assert_eq!(table_data.get_cell(0, 0).unwrap(), "test");

        let cell_err = table_data.get_cell(1, 0).unwrap_err();
        assert_eq!(cell_err.to_string(), "Invalid row or column index");
    }
}
