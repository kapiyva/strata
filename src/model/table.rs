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
            headers: vec!["0".to_string()],
            rows: vec![vec!["".to_string()]],
        })
    }

    pub fn from_csv(_file_path: PathBuf) -> Result<Self> {
        todo!()
    }

    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Result<&str> {
        if row >= self.rows.len() || col >= self.headers.len() {
            bail!("Invalid row or column index")
        }

        Ok(&self.rows[row][col])
    }

    pub fn update_header(&mut self, col: usize, value: &str) -> Result<()> {
        if col >= self.headers.len() {
            bail!("Invalid column index")
        }

        self.headers[col] = value.to_string();
        Ok(())
    }

    pub fn update_cell(&mut self, row: usize, col: usize, value: &str) -> Result<()> {
        if row >= self.rows.len() || col >= self.headers.len() {
            bail!("Invalid row or column index")
        }

        self.rows[row][col] = value.to_string();
        Ok(())
    }

    pub fn expand_row(&mut self) -> Result<()> {
        self.rows.push(vec!["".to_string(); self.headers.len()]);
        Ok(())
    }

    pub fn collapse_row(&mut self, row: usize) -> Result<()> {
        if row >= self.rows.len() {
            bail!("out of bounds")
        }
        self.rows.remove(row);
        Ok(())
    }

    pub fn expand_col(&mut self, header: &str) -> Result<()> {
        self.headers.push(header.to_string());
        for row in self.rows.iter_mut() {
            row.push("".to_string());
        }
        Ok(())
    }

    pub fn collapse_col(&mut self, col: usize) -> Result<()> {
        if col >= self.rows.len() {
            bail!("out of bounds")
        }
        self.headers.remove(col);
        for row in self.rows.iter_mut() {
            row.remove(col);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_table_data() -> TableData {
        TableData {
            headers: vec!["header0".to_string(), "header1".to_string()],
            rows: vec![
                vec!["cell00".to_string(), "cell01".to_string()],
                vec!["cell10".to_string(), "cell11".to_string()],
            ],
        }
    }

    #[test]
    fn test_table_name() {
        let table_name = TableName::from("test_table").unwrap();
        assert_eq!(table_name.as_str(), "test_table");

        let table_name_err = TableName::from("").unwrap_err();
        assert_eq!(table_name_err.to_string(), "Table name cannot be empty");
    }

    #[test]
    fn test_new_table() {
        let table_data = TableData::new().unwrap();
        assert_eq!(table_data.headers.len(), 1);
        assert_eq!(table_data.headers[0], "0");
        assert_eq!(table_data.rows.len(), 1);
    }

    #[test]
    fn test_expand_row() {
        let mut table_data = setup_table_data();
        // 3x2 table
        table_data.expand_row().unwrap();
        assert_eq!(table_data.rows.len(), 3);
        assert_eq!(table_data.rows[0][0], "cell00");
        assert_eq!(table_data.rows[2][1], "");
    }

    #[test]
    fn test_collapse_row() {
        let mut table_data = setup_table_data();
        // 1x2 table
        table_data.collapse_row(0).unwrap();
        assert_eq!(table_data.rows.len(), 1);
        assert_eq!(table_data.rows[0][0], "cell10");
        assert_eq!(table_data.rows[0][1], "cell11");
    }

    #[test]
    fn test_expand_col() {
        let mut table_data = setup_table_data();
        // 2x3 table
        table_data.expand_col("header1").unwrap();
        assert_eq!(table_data.headers.len(), 3);
        assert_eq!(table_data.headers[1], "header1");
        assert_eq!(table_data.rows[0][0], "cell00");
        assert_eq!(table_data.rows[1][2], "");
    }

    #[test]
    fn test_collapse_col() {
        let mut table_data = setup_table_data();
        // 2x1 table
        table_data.collapse_col(0).unwrap();
        assert_eq!(table_data.headers.len(), 1);
        assert_eq!(table_data.rows[0][0], "cell01");
        assert_eq!(table_data.rows[1][0], "cell11");
    }

    #[test]
    fn test_update_cell() {
        let mut table_data = setup_table_data();

        table_data.update_cell(0, 0, "test").unwrap();
        assert_eq!(table_data.get_cell(0, 0).unwrap(), "test");

        let cell_err = table_data.get_cell(2, 0).unwrap_err();
        assert_eq!(cell_err.to_string(), "Invalid row or column index");
    }
}
