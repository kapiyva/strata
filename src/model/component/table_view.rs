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

    pub fn switch_headers(self) -> Result<Self> {
        let has_header = !self.has_header;
        match has_header {
            true => {
                let mut rows = self.rows;
                let _ = rows.remove(0);
                Ok(Self {
                    has_header,
                    rows,
                    ..self
                })
            }
            false => {
                let mut rows = self.rows;
                rows.insert(0, self.header.clone());
                Ok(Self {
                    has_header,
                    rows,
                    ..self
                })
            }
        }
    }

    pub fn move_selector(self, row_move: isize, col_move: isize) -> Result<Self> {
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

        Ok(Self {
            cell_selector: self
                .cell_selector
                .with_selected_cell(Some((new_row, new_col))),
            ..self
        })
    }

    pub fn select_cell(self, row: usize, col: usize) -> Result<Self> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        Ok(Self {
            cell_selector: self.cell_selector.with_selected_cell(Some((row, col))),
            ..self
        })
    }

    pub fn update_header(self, col: usize, value: &str) -> Result<Self> {
        self.is_valid_col_index(col)?;
        if !self.has_header {
            bail!(StrataError::TableHasNoHeader);
        }

        let mut header = self.header;
        if let Some(h) = header.get_mut(col) {
            *h = value.to_string();
        }

        Ok(Self { header, ..self })
    }

    pub fn update_cell(self, row: usize, col: usize, value: &str) -> Result<Self> {
        self.is_valid_row_index(row)?;
        self.is_valid_col_index(col)?;

        let mut rows = self.rows;
        if let Some(r) = rows.get_mut(row) {
            if let Some(c) = r.get_mut(col) {
                *c = value.to_string();
            }
        }

        Ok(Self { rows, ..self })
    }

    pub fn expand_row(self) -> Result<Self> {
        let mut rows = self.rows;
        rows.push(vec!["".to_string(); self.header.len()]);

        Ok(Self { rows, ..self })
    }

    pub fn collapse_row(self, row: usize) -> Result<Self> {
        self.is_valid_row_index(row)?;

        let mut rows = self.rows;
        rows.remove(row);
        Ok(Self { rows, ..self })
    }

    pub fn expand_col(self, new_header: &str) -> Result<Self> {
        let mut header = self.header;
        header.push(new_header.to_string());
        let mut rows = self.rows;
        for row in rows.iter_mut() {
            row.push("".to_string());
        }

        Ok(Self {
            header,
            rows,
            ..self
        })
    }

    pub fn collapse_col(self, col: usize) -> Result<Self> {
        self.is_valid_col_index(col)?;

        let mut header = self.header;
        header.remove(col);

        let mut rows = self.rows;
        for row in rows.iter_mut() {
            row.remove(col);
        }

        Ok(Self {
            header,
            rows,
            ..self
        })
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
        let tv = TableView::new().switch_headers().unwrap();
        assert_eq!(tv.has_header, false);
        assert_eq!(tv.rows.get(0), Some(&tv.header));

        let tv = tv.switch_headers().unwrap();
        assert_eq!(tv.has_header, true);
        assert_eq!(tv.rows.get(0), Some(vec!["".to_string(); 10].as_ref()));
    }

    #[test]
    fn test_move_selector() {
        let tv = TableView::new().move_selector(1, 1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((1, 1)));

        let tv = tv.move_selector(1, 1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((2, 2)));

        let tv = tv.move_selector(-1, -1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((1, 1)));

        let tv = tv.move_selector(-1, -1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((0, 0)));

        let tv = tv.move_selector(-1, -1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((0, 0)));
    }

    #[test]
    fn test_select_cell() {
        let tv = TableView::new().select_cell(1, 1).unwrap();
        assert_eq!(tv.get_selector_index(), Some((1, 1)));

        let tv = tv.select_cell(2, 2).unwrap();
        assert_eq!(tv.get_selector_index(), Some((2, 2)));

        let tv = tv.select_cell(0, 0).unwrap();
        assert_eq!(tv.get_selector_index(), Some((0, 0)));
    }

    #[test]
    fn test_update_header() {
        let tv = TableView::new().update_header(1, "new_header").unwrap();
        assert_eq!(tv.header[1], "new_header");

        let tv = tv.update_header(0, "new_header").unwrap();
        assert_eq!(tv.header[0], "new_header");

        let tv = tv.switch_headers().unwrap().update_header(0, "new_header");
        assert!(tv.is_err());
    }

    #[test]
    fn test_update_cell() {
        let tv = TableView::new().update_cell(1, 1, "new_value").unwrap();
        assert_eq!(tv.rows[1][1], "new_value");

        let tv = tv.update_cell(0, 0, "new_value").unwrap();
        assert_eq!(tv.rows[0][0], "new_value");
    }

    #[test]
    fn test_expand_row() {
        let tv = TableView::new().expand_row().unwrap();
        assert_eq!(tv.rows.len(), INITIAL_TABLE_SIZE + 1);
        assert_eq!(tv.rows[INITIAL_TABLE_SIZE].len(), INITIAL_TABLE_SIZE);
    }

    #[test]
    fn test_collapse_row() {
        let tv = TableView::new().collapse_row(1).unwrap();
        assert_eq!(tv.rows.len(), INITIAL_TABLE_SIZE - 1);
    }

    #[test]
    fn test_expand_col() {
        let tv = TableView::new().expand_col("new_header").unwrap();
        assert_eq!(tv.header.len(), INITIAL_TABLE_SIZE + 1);
        assert_eq!(tv.rows[0].len(), INITIAL_TABLE_SIZE + 1);
    }

    #[test]
    fn test_collapse_col() {
        let tv = TableView::new().collapse_col(1).unwrap();
        assert_eq!(tv.header.len(), INITIAL_TABLE_SIZE - 1);
        assert_eq!(tv.rows[0].len(), INITIAL_TABLE_SIZE - 1);
    }
}
