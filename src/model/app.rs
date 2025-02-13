pub mod state;

use std::collections::HashMap;

use color_eyre::eyre::Result;
use eyre::{bail, OptionExt};
use state::{
    AddTableState, DisplayState, DisplayTableState, EditCellState, SelectTableState, SelectedCell,
};

use super::table::{TableData, TableName};

#[derive(Default)]
pub struct App {
    display_state: DisplayState,
    table_map: HashMap<TableName, TableData>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_display_state(&self) -> &DisplayState {
        &self.display_state
    }

    pub fn get_all_table_names(&self) -> Vec<&TableName> {
        self.table_map.keys().collect()
    }

    pub fn get_selected_table_name(&self) -> Option<&TableName> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState { selected_cell })
            | DisplayState::SelectTable(SelectTableState { selected_cell }) => selected_cell
                .as_ref()
                .map(|selected_cell| &selected_cell.table_name),

            DisplayState::DisplayTable(DisplayTableState { selected_cell })
            | DisplayState::EditCell(EditCellState { selected_cell }) => {
                Some(&selected_cell.table_name)
            }
        }
    }

    pub fn get_table_data(&self) -> Option<&TableData> {
        let table_name = &self.get_selected_cell()?.table_name;

        self.table_map.get(table_name)
    }

    pub fn get_selected_cell(&self) -> Option<&SelectedCell> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState { selected_cell })
            | DisplayState::SelectTable(SelectTableState { selected_cell }) => {
                selected_cell.as_ref()
            }

            DisplayState::DisplayTable(DisplayTableState { selected_cell })
            | DisplayState::EditCell(EditCellState { selected_cell }) => Some(selected_cell),
        }
    }

    /// Change the display state to AddTable
    pub fn set_state_add_table(&mut self) -> Result<()> {
        match &self.display_state {
            DisplayState::SelectTable(SelectTableState { selected_cell }) => {
                self.display_state = DisplayState::AddTable(AddTableState {
                    selected_cell: selected_cell.clone(),
                });
                Ok(())
            }
            _ => bail!("Cannot change add table state from current state"),
        }
    }

    /// Change the display state to SelectTable
    pub fn set_state_select_table(&mut self) -> Result<()> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState {
                selected_cell: Some(selected_cell),
            })
            | DisplayState::DisplayTable(DisplayTableState { selected_cell }) => {
                self.display_state = DisplayState::SelectTable(SelectTableState {
                    selected_cell: Some(selected_cell.clone()),
                });
                Ok(())
            }
            DisplayState::AddTable(AddTableState {
                selected_cell: None,
            }) => {
                self.display_state = DisplayState::SelectTable(SelectTableState {
                    selected_cell: None,
                });
                Ok(())
            }
            _ => bail!("Cannot change select table state from current state"),
        }
    }

    /// Change the display state to EditCell
    pub fn set_state_edit_cell(&mut self) -> Result<()> {
        // Only allow changing to EditCell state from DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState { selected_cell }) = &self.display_state
        else {
            bail!("Cannot change edit cell state from current state");
        };
        self.display_state = DisplayState::EditCell(EditCellState {
            selected_cell: selected_cell.clone(),
        });

        Ok(())
    }

    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        // Only allow adding table in AddTable state
        let DisplayState::AddTable(_) = self.display_state else {
            bail!("Cannot add table in current state");
        };

        let table_name = TableName::from(table_name_str)?;
        if self.table_map.contains_key(&table_name) {
            bail!("Table already exists");
        }

        self.table_map.insert(table_name.clone(), TableData::new()?);
        self.display_state = DisplayState::DisplayTable(DisplayTableState {
            selected_cell: SelectedCell::new(table_name),
        });
        Ok(())
    }

    pub fn select_table(&mut self, table_name_str: &str) -> Result<()> {
        let table_name = TableName::from(table_name_str)?;
        if !self.table_map.contains_key(&table_name) {
            bail!("Table does not exist");
        };

        match &self.display_state {
            DisplayState::SelectTable(SelectTableState {
                selected_cell: Some(already_selected_cell),
            }) => {
                // Only change the table if the selected table is different
                if already_selected_cell.table_name != table_name {
                    self.display_state = DisplayState::DisplayTable(DisplayTableState {
                        selected_cell: SelectedCell::new(table_name),
                    });
                } else {
                    self.display_state = DisplayState::DisplayTable(DisplayTableState {
                        selected_cell: already_selected_cell.clone(),
                    });
                }
            }
            DisplayState::SelectTable(SelectTableState {
                selected_cell: None,
            }) => {
                self.display_state = DisplayState::DisplayTable(DisplayTableState {
                    selected_cell: SelectedCell::new(table_name),
                });
            }
            _ => bail!("Cannot select table in current state"),
        };
        Ok(())
    }

    pub fn move_cursor(&mut self, row: isize, col: isize) -> Result<()> {
        // Only allow moving cursor in DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState { selected_cell }) = &self.display_state
        else {
            bail!("Cannot move cursor in current state");
        };

        let table_data = self.get_table_data()?;
        let new_row = (selected_cell.row as isize + row).max(0) as usize;
        let new_col = (selected_cell.col as isize + col).max(0) as usize;
        if new_row < table_data.rows.len() && new_col < table_data.headers.len() {
            self.display_state = DisplayState::DisplayTable(DisplayTableState {
                selected_cell: SelectedCell {
                    table_name: selected_cell.table_name.clone(),
                    row: new_row,
                    col: new_col,
                },
            });
        };

        Ok(())
    }

    pub fn jump_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        // Only allow jumping cursor in DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState { selected_cell }) = &self.display_state
        else {
            bail!("Cannot jump cursor in current state");
        };

        let table_data = self.get_table_data()?;
        if row < table_data.rows.len() && col < table_data.headers.len() {
            self.display_state = DisplayState::DisplayTable(DisplayTableState {
                selected_cell: SelectedCell {
                    table_name: selected_cell.table_name.clone(),
                    row,
                    col,
                },
            });
        };

        Ok(())
    }

    pub fn get_cell_value(&self) -> Result<String> {
        match &self.display_state {
            DisplayState::DisplayTable(DisplayTableState { selected_cell })
            | DisplayState::EditCell(EditCellState { selected_cell }) => {
                let table_data = self.get_table_data()?;
                Ok(table_data.rows[selected_cell.row][selected_cell.col].clone())
            }
            _ => bail!("Cannot get cell value in current state"),
        }
    }

    pub fn update_cell_value(&mut self, value: &str) -> Result<()> {
        let DisplayState::EditCell(EditCellState { selected_cell }) = &self.display_state else {
            bail!("Cannot update cell value in current state");
        };
        let table_data = self
            .table_map
            .get_mut(&selected_cell.table_name)
            .ok_or_eyre(format!(
                "Table not found: {}",
                selected_cell.table_name.as_str()
            ))?;
        table_data.rows[selected_cell.row][selected_cell.col] = value.to_string();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_table() {
        let mut app = App::new();
        app.add_table("table1").unwrap();
        assert_eq!(
            app.get_all_table_names(),
            vec![&TableName::from("table1").unwrap()]
        );
    }

    #[test]
    fn test_select_table() {
        let mut app = App::new();
        app.add_table("table1").unwrap();
        app.add_table("table2").unwrap();
        app.select_table("table1").unwrap();
        assert_eq!(
            app.get_selected_table_name(),
            Some(&TableName::from("table1").unwrap())
        );
        app.select_table("table2").unwrap();
        assert_eq!(
            app.get_selected_table_name(),
            Some(&TableName::from("table2").unwrap())
        );
    }

    #[test]
    fn test_move_cursor() {
        let mut app = App::new();
        app.add_table("table1").unwrap();
        app.update_cell_value("value1").unwrap();
        app.move_cursor(1, 0).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1");
        app.move_cursor(0, 1).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1");
        app.move_cursor(-1, 0).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1");
        app.move_cursor(0, -1).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1");
    }

    #[test]
    fn test_jump_cursor() {
        let mut app = App::new();
        app.add_table("table1").unwrap();
        app.update_cell_value("value1").unwrap();
        app.jump_cursor(1, 0).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1");
        app.jump_cursor(0, 1).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1");
    }

    #[test]
    fn test_set_state_add_table() {
        let mut app = App::new();
        app.set_state_add_table().unwrap();
        assert!(matches!(app.get_display_state(), DisplayState::AddTable(_)));
    }
}
