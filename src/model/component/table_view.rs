use std::path::Path;

use eyre::{bail, OptionExt, Result};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::error::StrataError;

use super::StrataComponent;

pub const INITIAL_TABLE_SIZE: usize = 10;

#[derive(Default)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct TableView {
    pub has_header: bool,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub cell_selector: TableState,
}

impl TableView {
    pub fn new() -> Self {
        Self {
            has_header: true,
            header: (0..(INITIAL_TABLE_SIZE))
                .map(|i| format!("header{}", i))
                .collect(),
            rows: vec![vec!["".to_string(); INITIAL_TABLE_SIZE]; INITIAL_TABLE_SIZE],
            cell_selector: TableState::default().with_selected_cell(Some((0, 0))),
        }
    }

    pub fn from_csv(file_path: &Path, has_header: bool) -> Result<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(has_header)
            .from_path(file_path)?;

        let header: Vec<String> = if has_header {
            reader.headers()?.iter().map(|s| s.to_string()).collect()
        } else {
            (0..(reader.headers()?.len()))
                .map(|i| format!("header{}", i))
                .collect()
        };

        let mut rows = Vec::<Vec<String>>::new();
        while let Some(record) = reader.records().next() {
            rows.push(record?.iter().map(|s| s.to_string()).collect());
        }

        Ok(Self {
            has_header,
            header,
            rows,
            cell_selector: TableState::default().with_selected_cell(Some((0, 0))),
        })
    }

    pub fn get_header(&self) -> &Vec<String> {
        &self.header
    }

    pub fn get_selector_index(&self) -> Option<(usize, usize)> {
        self.cell_selector.selected_cell()
    }

    pub fn get_cell_value(&self, row: usize, col: usize) -> Result<&str> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        Ok(&self.rows[row][col])
    }

    pub fn get_header_widths(&self) -> Vec<Constraint> {
        vec![Constraint::Length(3)]
            .into_iter()
            .chain(self.header.iter().enumerate().map(|(index, header)| {
                let max_row_width = self
                    .rows
                    .iter()
                    .map(|row| row.get(index).map(String::len).unwrap_or(0))
                    .max()
                    .unwrap_or(0);
                Constraint::Length(max_row_width.max(header.len()) as u16)
            }))
            .collect()
    }

    pub fn switch_headers(&mut self) -> Result<&mut Self> {
        let has_header = !self.has_header;
        match has_header {
            true => {
                // let mut rows = self.rows;
                self.rows.remove(0);
                Ok(self)
            }
            false => {
                // let mut rows = self.rows;
                self.rows.insert(0, self.header.clone());
                Ok(self)
            }
        }
    }

    pub fn move_selector(&mut self, row_move: isize, col_move: isize) -> Result<&mut Self> {
        let (selected_row, selected_col) = self
            .cell_selector
            .selected_cell()
            .ok_or_eyre(StrataError::NoCellSelected)?;
        let new_row = selected_row
            .saturating_add_signed(row_move)
            .min(self.max_row_index());
        let new_col = selected_col
            .saturating_add_signed(col_move)
            .min(self.max_col_index());

        self.cell_selector.select_cell(Some((new_row, new_col)));
        Ok(self)
    }

    pub fn select_cell(&mut self, row: usize, col: usize) -> Result<&mut Self> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        self.cell_selector.select_cell(Some((row, col)));
        Ok(self)
    }

    pub fn update_header(&mut self, col: usize, value: &str) -> Result<&mut Self> {
        self.is_valid_col_index(col)?;
        if !self.has_header {
            bail!(StrataError::TableHasNoHeader);
        }

        if let Some(h) = self.header.get_mut(col) {
            *h = value.to_string();
        }

        Ok(self)
    }

    pub fn update_cell(&mut self, row: usize, col: usize, value: &str) -> Result<&mut Self> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        if let Some(r) = self.rows.get_mut(row) {
            if let Some(c) = r.get_mut(col) {
                *c = value.to_string();
            }
        }

        Ok(self)
    }

    pub fn expand_row(&mut self) -> Result<&mut Self> {
        self.rows.push(vec!["".to_string(); self.header.len()]);

        Ok(self)
    }

    pub fn collapse_row(&mut self, row: usize) -> Result<&mut Self> {
        self.is_valid_row_index(row)?;

        self.rows.remove(row);
        Ok(self)
    }

    pub fn expand_col(&mut self, new_header: &str) -> Result<&mut Self> {
        self.header.push(new_header.to_string());
        for row in self.rows.iter_mut() {
            row.push("".to_string());
        }

        Ok(self)
    }

    pub fn collapse_col(&mut self, col: usize) -> Result<&mut Self> {
        self.is_valid_col_index(col)?;

        self.header.remove(col);
        for row in self.rows.iter_mut() {
            row.remove(col);
        }

        Ok(self)
    }

    fn max_row_index(&self) -> usize {
        self.rows.len().saturating_sub(1)
    }

    fn max_col_index(&self) -> usize {
        self.header.len().saturating_sub(1)
    }

    fn is_valid_row_index(&self, row: usize) -> Result<()> {
        if row > self.max_row_index() {
            bail!(StrataError::InvalidRowIndex {
                max: self.max_row_index(),
                requested: row,
            });
        }
        Ok(())
    }

    fn is_valid_col_index(&self, col: usize) -> Result<()> {
        if col > self.max_col_index() {
            bail!(StrataError::InvalidColumnIndex {
                max: self.max_col_index(),
                requested: col,
            });
        }
        Ok(())
    }
}

impl StrataComponent for TableView {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let (selected_row, selected_col) = self
            .get_selector_index()
            .map(|(row, col)| (Some(row), Some(col)))
            .unwrap_or((None, None));

        let header_style = |col: usize| -> Style {
            if Some(col) == selected_col {
                return Style::default().bold().bg(Color::LightBlue);
            };
            Style::default().bold()
        };

        let header = Row::new(
            std::iter::once(Cell::from("#")).chain(
                self.header
                    .iter()
                    .enumerate()
                    .map(|(col, header)| Cell::from(header.clone()).style(header_style(col))),
            ),
        )
        .bottom_margin(1);

        let index_style = |row: usize| -> Style {
            if Some(row) == selected_row {
                return Style::default().bg(Color::LightBlue);
            };
            Style::default()
        };
        let cell_style = |(row, col): (usize, usize)| -> Style {
            if Some((row, col)) == self.get_selector_index() {
                return Style::default().bg(Color::LightBlue);
            };
            Style::default()
        };

        let body = self.rows.iter().enumerate().map(|(row_index, row)| {
            Row::new(
                std::iter::once(Cell::from(row_index.to_string()).style(index_style(row_index)))
                    .chain(row.iter().enumerate().map(|(col_index, cell_value)| {
                        Cell::from(cell_value.clone()).style(cell_style((row_index, col_index)))
                    })),
            )
        });

        let table = Table::new(body, self.get_header_widths())
            .block(Block::default().title("Table").borders(Borders::ALL))
            .style(if is_focused {
                Style::default().bold().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .header(header);

        frame.render_stateful_widget(table, area, &mut self.cell_selector.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tv = TableView::new();
        assert_eq!(tv.header.len(), INITIAL_TABLE_SIZE);
        assert_eq!(tv.header[0], "header0");
        assert_eq!(tv.header[9], "header9");
        assert_eq!(tv.rows.len(), INITIAL_TABLE_SIZE);
        assert_eq!(tv.rows[0].len(), INITIAL_TABLE_SIZE);
    }

    #[test]
    fn test_from_csv() {
        // has_header = true
        let tv = TableView::from_csv(Path::new("tests/data/fluits.csv"), true).unwrap();
        println!("{:?}", tv);
        assert_eq!(tv.header.len(), 2);
        assert_eq!(tv.header[0], "item");
        assert_eq!(tv.header[1], "price");
        assert_eq!(tv.rows.len(), 3);
        assert_eq!(tv.rows[0][0], "apple".to_string());
        assert_eq!(tv.rows[0][1], "100".to_string());
        assert_eq!(tv.rows[1][0], "orange".to_string());
        assert_eq!(tv.rows[2][1], "150".to_string());

        // has_header = false
        let tv = TableView::from_csv(Path::new("tests/data/fluits.csv"), false).unwrap();
        println!("{:?}", tv);
        assert_eq!(tv.header.len(), 2);
        assert_eq!(tv.header[0], "header0");
        assert_eq!(tv.header[1], "header1");
        assert_eq!(tv.rows.len(), 4);
        assert_eq!(tv.rows[0][0], "item".to_string());
        assert_eq!(tv.rows[0][1], "price".to_string());
        assert_eq!(tv.rows[1][0], "apple".to_string());
        assert_eq!(tv.rows[1][1], "100".to_string());
        assert_eq!(tv.rows[2][0], "orange".to_string());
        assert_eq!(tv.rows[3][1], "150".to_string());
    }

    #[test]
    fn test_switch_headers() {
        let mut tv = TableView::new();
        tv.switch_headers().unwrap();
        assert_eq!(tv.has_header, false);
        assert_eq!(tv.rows.get(0), Some(&tv.header));

        let tv = tv.switch_headers().unwrap();
        assert_eq!(tv.has_header, true);
        assert_eq!(tv.rows.get(0), Some(vec!["".to_string(); 10].as_ref()));
    }

    #[test]
    fn test_move_selector() {
        let mut tv = TableView::new();
        tv.move_selector(1, 1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((1, 1)));

        tv.move_selector(1, 1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((2, 2)));

        tv.move_selector(-1, -1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((1, 1)));

        tv.move_selector(-1, -1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((0, 0)));

        tv.move_selector(-1, -1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((0, 0)));
    }

    #[test]
    fn test_select_cell() {
        let mut tv = TableView::new();
        tv.select_cell(1, 1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((1, 1)));

        tv.select_cell(2, 2).unwrap();
        assert_eq!(tv.get_selector_index(), Some((2, 2)));

        tv.select_cell(0, 0).unwrap();
        assert_eq!(tv.get_selector_index(), Some((0, 0)));
    }

    #[test]
    fn test_update_header() {
        let mut tv = TableView::new();
        tv.update_header(1, "new_header").unwrap();
        assert_eq!(tv.header[1], "new_header");

        tv.update_header(0, "new_header").unwrap();
        assert_eq!(tv.header[0], "new_header");

        let tv = tv
            .switch_headers()
            .and_then(|tv| tv.update_header(0, "new_header"));
        assert!(tv.is_err());
    }

    #[test]
    fn test_update_cell() {
        let mut tv = TableView::new();
        tv.update_cell(1, 1, "new_value").unwrap();
        assert_eq!(tv.rows[1][1], "new_value");

        tv.update_cell(0, 0, "new_value").unwrap();
        assert_eq!(tv.rows[0][0], "new_value");
    }

    #[test]
    fn test_expand_row() {
        let mut tv = TableView::new();
        tv.expand_row().unwrap();

        assert_eq!(tv.rows.len(), INITIAL_TABLE_SIZE + 1);
        assert_eq!(tv.rows[INITIAL_TABLE_SIZE].len(), INITIAL_TABLE_SIZE);
    }

    #[test]
    fn test_collapse_row() {
        let mut tv = TableView::new();
        tv.collapse_row(1).unwrap();

        assert_eq!(tv.rows.len(), INITIAL_TABLE_SIZE - 1);
    }

    #[test]
    fn test_expand_col() {
        let mut tv = TableView::new();
        tv.expand_col("new_header").unwrap();

        assert_eq!(tv.header.len(), INITIAL_TABLE_SIZE + 1);
        assert_eq!(tv.rows[0].len(), INITIAL_TABLE_SIZE + 1);
    }

    #[test]
    fn test_collapse_col() {
        let mut tv = TableView::new();
        tv.collapse_col(1).unwrap();

        assert_eq!(tv.header.len(), INITIAL_TABLE_SIZE - 1);
        assert_eq!(tv.rows[0].len(), INITIAL_TABLE_SIZE - 1);
    }
}
