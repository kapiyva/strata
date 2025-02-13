mod state;

use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use eyre::bail;
use state::{AddTableState, DisplayState, DisplayTableState, EditCellState, SelectTableState, SelectedCell};

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

    pub fn get_table_names(&self) -> Vec<&TableName> {
        self.table_map.keys().collect()
    }

    pub fn get_table_data(&self, table_name: &str) -> Result<Option<&TableData>> {
        Ok(self.table_map.get(&TableName::from(table_name)?))
    }

    pub fn get_selected_cell(&self) -> Option<&SelectedCell> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState { selected_cell }) => selected_cell.as_ref(),
            DisplayState::SelectTable(SelectTableState { selected_cell }) => selected_cell.as_ref(),
            DisplayState::DisplayTable(DisplayTableState { selected_cell }) => Some(selected_cell),
            DisplayState::EditCell(EditCellState { selected_cell }) => Some(selected_cell),
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
    pub fn set_state_edit_cell(&mut self, row: usize, col: usize) -> Result<()> {
        match &self.display_state {
            DisplayState::DisplayTable(DisplayTableState { selected_cell }) => {
                self.display_state = DisplayState::EditCell(EditCellState {
                    selected_cell: SelectedCell {
                        table_name: selected_cell.table_name.clone(),
                        row,
                        col,
                    },
                });
                Ok(())
            }
            _ => bail!("Cannot change edit cell state from current state"),
        }
    }

    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        match self.display_state {
            DisplayState::AddTable(_) => {
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
            _ => bail!("Cannot add table in current state"),
        }
    }

    pub fn select_table(&mut self, table_name_str: &str) -> Result<()> {
        let table_name = TableName::from(table_name_str)?;
        match &self.display_state {
            DisplayState::SelectTable(SelectTableState {
                selected_cell: Some(already_selected_cell),
            }) => {
                if !self.table_map.contains_key(&table_name) {
                    bail!("Table does not exist");
                }
                // Only change the table if the selected table is different
                if already_selected_cell.table_name != table_name {
                    self.display_state = DisplayState::DisplayTable(DisplayTableState {
                        selected_cell: SelectedCell::new(table_name),
                    });
                }
                Ok(())
            }
            DisplayState::SelectTable(SelectTableState {
                selected_cell: None,
            }) => {
                if !self.table_map.contains_key(&table_name) {
                    bail!("Table does not exist");
                }

                self.display_state = DisplayState::DisplayTable(DisplayTableState {
                    selected_cell: SelectedCell::new(table_name),
                });
                Ok(())
            }
            _ => bail!("Cannot select table in current state"),
        }
    }

    pub fn move_cursor(&mut self, row: isize, col: isize) -> Result<()> {
        match &self.display_state {
            DisplayState::DisplayTable(DisplayTableState { selected_cell }) => {
                let table_data =
                    self.table_map
                        .get(&selected_cell.table_name)
                        .ok_or_else(|| {
                            eyre!("Table not found: {}", selected_cell.table_name.as_str())
                        })?;
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
                }
                Ok(())
            }
            _ => bail!("Cannot move cursor in current state"),
        }
    }

    pub fn jump_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        match &self.display_state {
            DisplayState::DisplayTable(DisplayTableState { selected_cell }) => {
                let table_data =
                    self.table_map
                        .get(&selected_cell.table_name)
                        .ok_or_else(|| {
                            eyre!("Table not found: {}", selected_cell.table_name.as_str())
                        })?;
                if row < table_data.rows.len() && col < table_data.headers.len() {
                    self.display_state = DisplayState::DisplayTable(DisplayTableState {
                        selected_cell: SelectedCell {
                            table_name: selected_cell.table_name.clone(),
                            row,
                            col,
                        },
                    });
                }
                Ok(())
            }
            _ => bail!("Cannot jump cursor in current state"),
        }
    }

    pub fn update_cell_value(&mut self, value: &str) -> Result<()> {
        match &self.display_state {
            DisplayState::EditCell(EditCellState { selected_cell }) => {
                let table_data = self
                    .table_map
                    .get_mut(&selected_cell.table_name)
                    .ok_or_else(|| {
                        eyre!("Table not found: {}", selected_cell.table_name.as_str())
                    })?;
                table_data.rows[selected_cell.row][selected_cell.col] = value.to_string();
                Ok(())
            }
            _ => bail!("Cannot update cell value in current state"),
        }
    }
}


