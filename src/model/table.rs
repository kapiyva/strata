use std::path::Path;

use eyre::{bail, OptionExt, Result};
use ratatui::widgets::TableState;

use crate::error::StrataError;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TableName(String);

impl ToString for TableName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TableName {
    pub fn from<T>(_name: T) -> Result<TableName>
    where
        T: ToString,
    {
        let name = _name.to_string();
        if name.is_empty() {
            bail!(StrataError::InvalidTableName)
        } else {
            Ok(Self(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub const INITIAL_TABLE_SIZE: usize = 10;
pub const INITIAL_TABLE_NAME: &str = "new_table";

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, PartialEq))]
pub struct TableData {
    pub no_headers: bool,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub table_view_state: TableState,
}

impl Default for TableData {
    fn default() -> Self {
        Self {
            no_headers: false,
            headers: vec!["header0".to_string()],
            rows: vec![vec!["".to_string(); 1]],
            table_view_state: TableState::default(),
        }
    }
}

impl TableData {
    pub fn new() -> Self {
        Self {
            no_headers: false,
            headers: (0..(INITIAL_TABLE_SIZE))
                .map(|i| format!("header{}", i))
                .collect(),
            rows: vec![vec!["".to_string(); INITIAL_TABLE_SIZE]; INITIAL_TABLE_SIZE],
            table_view_state: TableState::default().with_selected_cell(Some((0, 0))),
        }
    }

    pub fn from_csv(file_path: &Path) -> Result<Self> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let mut headers = Vec::new();
        let mut rows = Vec::<Vec<String>>::new();
        while let Some(record) = reader.records().next() {
            let record = record?;
            if headers.is_empty() {
                headers = record.iter().map(|s| s.to_string()).collect();
            } else {
                rows.push(record.iter().map(|s| s.to_string()).collect());
            }
        }

        Ok(Self {
            no_headers: false,
            headers,
            rows,
            table_view_state: TableState::default().with_selected_cell(Some((0, 0))),
        })
    }

    pub fn switch_headers(&mut self) {
        self.no_headers = !self.no_headers;
        match self.no_headers {
            true => self.headers = self.rows.remove(0),
            false => {
                self.rows.insert(0, self.headers.clone());
            }
        }
    }

    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }

    pub fn get_max_row_index(&self) -> usize {
        self.rows.len() - 1
    }

    pub fn get_max_col_index(&self) -> usize {
        self.headers.len() - 1
    }

    pub fn get_selector_index(&self) -> Option<(usize, usize)> {
        self.table_view_state.selected_cell()
    }

    pub fn move_selector(&mut self, row_move: isize, col_move: isize) -> Result<()> {
        let (max_row, max_col) = { (self.get_max_row_index(), self.get_max_col_index()) };
        let (selected_row, selected_col) = self
            .table_view_state
            .selected_cell()
            .ok_or_eyre(StrataError::NoCellSelected)?;
        let new_row = selected_row.saturating_add_signed(row_move).min(max_row);
        let new_col = selected_col.saturating_add_signed(col_move).min(max_col);

        self.table_view_state.select_cell(Some((new_row, new_col)));
        Ok(())
    }

    pub fn jump_selector(&mut self, row: usize, col: usize) -> Result<()> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        self.table_view_state.select_cell(Some((row, col)));
        Ok(())
    }

    pub fn get_cell_value(&self, row: usize, col: usize) -> Result<&str> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        Ok(&self.rows[row][col])
    }

    pub fn select_cell(&mut self, row: usize, col: usize) -> Result<()> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        self.table_view_state.select_cell(Some((row, col)));
        Ok(())
    }

    pub fn update_header(&mut self, col: usize, value: &str) -> Result<()> {
        self.is_valid_col_index(col)?;

        self.headers[col] = value.to_string();
        Ok(())
    }

    pub fn update_cell(&mut self, row: usize, col: usize, value: &str) -> Result<()> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        self.rows[row][col] = value.to_string();
        Ok(())
    }

    pub fn expand_row(&mut self) -> Result<()> {
        self.rows.push(vec!["".to_string(); self.headers.len()]);
        Ok(())
    }

    pub fn collapse_row(&mut self, row: usize) -> Result<()> {
        self.is_valid_row_index(row)?;

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
        self.is_valid_col_index(col)?;

        self.headers.remove(col);
        for row in self.rows.iter_mut() {
            row.remove(col);
        }
        Ok(())
    }

    pub fn is_valid_row_index(&self, row: usize) -> Result<()> {
        if row >= self.rows.len() {
            bail!(StrataError::InvalidRowIndex {
                max: self.rows.len() - 1,
                requested: row,
            })
        }
        Ok(())
    }

    pub fn is_valid_col_index(&self, col: usize) -> Result<()> {
        if col >= self.headers.len() {
            bail!(StrataError::InvalidColumnIndex {
                max: self.headers.len() - 1,
                requested: col,
            })
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_table_data() -> TableData {
        TableData {
            no_headers: false,
            headers: vec!["header0".to_string(), "header1".to_string()],
            rows: vec![
                vec!["cell00".to_string(), "cell01".to_string()],
                vec!["cell10".to_string(), "cell11".to_string()],
            ],
            table_view_state: TableState::default().with_selected_cell(Some((0, 0))),
        }
    }

    #[test]
    fn test_table_name() {
        let table_name = TableName::from("test_table").unwrap();
        assert_eq!(table_name.as_str(), "test_table");

        let table_name_err = TableName::from("").unwrap_err();
        assert_eq!(
            table_name_err.to_string(),
            StrataError::InvalidTableName.to_string()
        );
    }

    #[test]
    fn test_new_table() {
        let table_data = TableData::new();
        assert_eq!(table_data.headers.len(), INITIAL_TABLE_SIZE);
        assert_eq!(table_data.headers[0], "header0");
        assert_eq!(table_data.headers[9], "header9");
        assert_eq!(table_data.rows.len(), INITIAL_TABLE_SIZE);
    }

    #[test]
    fn test_open_csv() {
        let file_path = Path::new("tests/data/test_table.csv");
        let table_data = TableData::from_csv(file_path).unwrap();
        assert_eq!(table_data.headers.len(), 3);
        assert_eq!(table_data.headers[0], "header0");
        assert_eq!(table_data.headers[2], "header2");
        assert_eq!(table_data.rows.len(), 2);
        assert_eq!(table_data.rows[0][0], "cell00");
        assert_eq!(table_data.rows[1][2], "cell12");
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

        let row_err = table_data.collapse_row(1).unwrap_err();
        assert_eq!(
            row_err.to_string(),
            StrataError::InvalidRowIndex {
                max: 0,
                requested: 1
            }
            .to_string()
        );
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

        let col_err = table_data.collapse_col(1).unwrap_err();
        assert_eq!(
            col_err.to_string(),
            StrataError::InvalidColumnIndex {
                max: 0,
                requested: 1
            }
            .to_string()
        );
    }

    #[test]
    fn test_update_cell() {
        let mut table_data = setup_table_data();

        table_data.update_cell(0, 0, "test").unwrap();
        assert_eq!(table_data.rows[0][0], "test");
    }

    #[test]
    fn test_get_cell() {
        let table_data = setup_table_data();

        assert_eq!(table_data.get_cell_value(0, 0).unwrap(), "cell00");
        assert_eq!(table_data.get_cell_value(1, 1).unwrap(), "cell11");

        // Invalid row index
        assert_eq!(
            table_data.get_cell_value(2, 0).unwrap_err().to_string(),
            StrataError::InvalidRowIndex {
                max: 1,
                requested: 2
            }
            .to_string()
        );

        // Invalid column index
        assert_eq!(
            table_data.get_cell_value(0, 2).unwrap_err().to_string(),
            StrataError::InvalidColumnIndex {
                max: 1,
                requested: 2
            }
            .to_string()
        );
    }
}
