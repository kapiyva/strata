pub mod state;

use std::collections::HashMap;

use color_eyre::eyre::Result;
use eyre::{bail, OptionExt};
use state::*;

use crate::error::StrataError;

use super::table::{TableData, TableName};

#[derive(Default)]
pub struct App {
    display_state: DisplayState,
    table_map: HashMap<TableName, TableData>,
}

impl App {
    /// Setup a new App as SelectTable state
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_display_state(&self) -> &DisplayState {
        &self.display_state
    }

    pub fn get_all_table_names(&self) -> &Vec<TableName> {
        match &self.display_state {
            DisplayState::DisplayTable(DisplayTableState { table_list, .. })
            | DisplayState::EditCell(EditCellState { table_list, .. })
            | DisplayState::AddTable(AddTableState { table_list, .. })
            | DisplayState::SelectTable(SelectTableState { table_list, .. }) => table_list,
        }
    }

    pub fn get_selected_table_name(&self) -> Option<&TableName> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState { selected_cell, .. })
            | DisplayState::SelectTable(SelectTableState { selected_cell, .. }) => {
                selected_cell.as_ref().map(|sc| &sc.table_name)
            }

            DisplayState::DisplayTable(DisplayTableState { selected_cell, .. })
            | DisplayState::EditCell(EditCellState { selected_cell, .. }) => {
                Some(&selected_cell.table_name)
            }
        }
    }

    pub fn get_selected_cell(&self) -> Option<&SelectedCell> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState { selected_cell, .. })
            | DisplayState::SelectTable(SelectTableState { selected_cell, .. }) => {
                selected_cell.as_ref()
            }

            DisplayState::DisplayTable(DisplayTableState { selected_cell, .. })
            | DisplayState::EditCell(EditCellState { selected_cell, .. }) => Some(selected_cell),
        }
    }

    pub fn get_table_data(&self) -> Result<&TableData> {
        let table_name = self
            .get_selected_table_name()
            .ok_or_eyre(StrataError::NoTableSelected)?;

        self.table_map
            .get(table_name)
            .ok_or_eyre(StrataError::TableNotFound(table_name.to_string()))
    }

    /// Call from SelectTable state
    /// Change the display state to AddTable state
    pub fn set_state_add_table(&mut self) -> Result<()> {
        match &self.display_state {
            DisplayState::SelectTable(SelectTableState {
                selected_cell,
                table_list,
                ..
            }) => {
                self.display_state = DisplayState::AddTable(AddTableState {
                    selected_cell: selected_cell.clone(),
                    table_list: table_list.clone(),
                });
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "set state AddTable".to_string(),
                state: self.display_state.to_string()
            }),
        }
    }

    /// Call from AddTable or DisplayTable state
    /// Change the display state to SelectTable
    pub fn set_state_select_table(&mut self) -> Result<()> {
        match &self.display_state {
            DisplayState::AddTable(AddTableState {
                selected_cell: Some(selected_cell),
                table_list,
            })
            | DisplayState::DisplayTable(DisplayTableState {
                selected_cell,
                table_list,
            }) => {
                self.display_state = DisplayState::SelectTable(SelectTableState {
                    selected_cell: Some(selected_cell.clone()),
                    table_list: table_list.clone(),
                    cursor: table_list
                        .iter()
                        .position(|tn| tn == &selected_cell.table_name)
                        .unwrap_or(0),
                });
                Ok(())
            }
            DisplayState::AddTable(AddTableState {
                selected_cell: None,
                table_list,
            }) => {
                self.display_state = DisplayState::SelectTable(SelectTableState {
                    selected_cell: None,
                    table_list: table_list.clone(),
                    cursor: 0,
                });
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "set state SelectTable".to_string(),
                state: self.display_state.to_string()
            }),
        }
    }

    /// Call from DisplayTable state
    /// Change the display state to EditCell
    pub fn set_state_edit_cell(&mut self) -> Result<()> {
        // Only allow changing to EditCell state from DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState {
            selected_cell,
            table_list,
        }) = &self.display_state
        else {
            bail!(StrataError::InvalidOperationCall {
                operation: "set state EditCell".to_string(),
                state: self.display_state.to_string()
            });
        };
        self.display_state = DisplayState::EditCell(EditCellState {
            selected_cell: selected_cell.clone(),
            table_list: table_list.clone(),
        });

        Ok(())
    }

    /// Call from AddTable state
    /// Add new table and change to DisplayTable state
    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        // Only allow adding table in AddTable state
        let DisplayState::AddTable(AddTableState { table_list, .. }) = &mut self.display_state
        else {
            bail!(StrataError::InvalidOperationCall {
                operation: "add table".to_string(),
                state: self.display_state.to_string()
            });
        };

        let table_name = TableName::from(table_name_str)?;
        if self.table_map.contains_key(&table_name) {
            bail!(StrataError::TableNameDuplicate(table_name_str.to_string()));
        }

        self.table_map.insert(table_name.clone(), TableData::new()?);
        table_list.push(table_name.clone());
        self.display_state = DisplayState::DisplayTable(DisplayTableState {
            selected_cell: SelectedCell::new(table_name),
            table_list: table_list.clone(),
        });
        Ok(())
    }

    /// Call from SelectTable state
    /// Move the table selector down
    pub fn down_table_selector(&mut self) -> Result<()> {
        match &mut self.display_state {
            DisplayState::SelectTable(SelectTableState {
                cursor, table_list, ..
            }) => {
                if *cursor < table_list.len() - 1 {
                    *cursor += 1;
                } else {
                    *cursor = table_list.len();
                }
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "down table selector".to_string(),
                state: self.display_state.to_string()
            }),
        }
    }

    /// Call from SelectTable state
    /// Move the table selector up
    pub fn up_table_selector(&mut self) -> Result<()> {
        match &mut self.display_state {
            DisplayState::SelectTable(SelectTableState { cursor, .. }) => {
                if *cursor > 0 {
                    *cursor -= 1;
                } else {
                    *cursor = 0;
                }
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "up table selector".to_string(),
                state: self.display_state.to_string()
            }),
        }
    }

    /// Call from SelectTable state
    /// Select table and change to DisplayTable state
    pub fn select_table(&mut self) -> Result<()> {
        match &self.display_state {
            DisplayState::SelectTable(SelectTableState {
                selected_cell: Some(already_selected_cell),
                table_list,
                cursor,
            }) => {
                if table_list.is_empty() {
                    bail!(StrataError::TableNotFound("".to_string()));
                }
                let table_name = table_list[*cursor].clone();
                // Only change the table if the selected table is different
                if already_selected_cell.table_name != table_name {
                    self.display_state = DisplayState::DisplayTable(DisplayTableState {
                        selected_cell: SelectedCell::new(table_name),
                        table_list: table_list.clone(),
                    });
                } else {
                    self.display_state = DisplayState::DisplayTable(DisplayTableState {
                        selected_cell: already_selected_cell.clone(),
                        table_list: table_list.clone(),
                    });
                }
            }
            DisplayState::SelectTable(SelectTableState {
                selected_cell: None,
                table_list,
                cursor,
            }) => {
                if table_list.is_empty() {
                    bail!(StrataError::TableNotFound("".to_string()));
                }
                let table_name = table_list[*cursor].clone();
                self.display_state = DisplayState::DisplayTable(DisplayTableState {
                    selected_cell: SelectedCell::new(table_name),
                    table_list: table_list.clone(),
                });
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "select table".to_string(),
                state: self.display_state.to_string()
            }),
        };
        Ok(())
    }

    /// Call from DisplayTable state
    /// Remove the selected table
    pub fn remove_table(&mut self) -> Result<()> {
        // remove table from table_map
        let table_name = self
            .get_selected_table_name()
            .ok_or_eyre(StrataError::NoTableSelected)?
            .clone();
        self.table_map.remove(&table_name);
        // remove table from table_list
        let mut table_list;
        {
            match &self.display_state {
                DisplayState::DisplayTable(DisplayTableState { table_list: tl, .. })
                | DisplayState::EditCell(EditCellState { table_list: tl, .. })
                | DisplayState::AddTable(AddTableState { table_list: tl, .. })
                | DisplayState::SelectTable(SelectTableState { table_list: tl, .. }) => {
                    table_list = tl.clone();
                }
            }
        }
        if table_list.is_empty() {
            bail!(StrataError::TableNotFound(table_name.to_string()));
        }

        table_list.retain(|tn| tn != &table_name);
        self.display_state = DisplayState::SelectTable(SelectTableState {
            selected_cell: Some(SelectedCell::new(table_list[0].clone())),
            table_list,
            cursor: 0,
        });

        Ok(())
    }

    /// Call from DisplayTable display state
    /// Move the cursor in the table
    pub fn move_cell_selector(&mut self, row: isize, col: isize) -> Result<()> {
        let selected_cell;
        let table_list;
        {
            // Only allow moving cursor in DisplayTable state
            let DisplayState::DisplayTable(DisplayTableState {
                selected_cell: sc,
                table_list: tl,
            }) = &self.display_state
            else {
                bail!(StrataError::InvalidOperationCall {
                    operation: "move cursor".to_string(),
                    state: self.display_state.to_string()
                });
            };

            selected_cell = sc.clone();
            table_list = tl.clone();
        }

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
                table_list,
            });
        }

        Ok(())
    }

    /// Call from DisplayTable state
    /// Jump the cursor to the specified cell
    pub fn jump_cell_selector(&mut self, row: usize, col: usize) -> Result<()> {
        // Only allow jumping cursor in DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState {
            selected_cell,
            table_list,
        }) = &self.display_state
        else {
            bail!(StrataError::InvalidOperationCall {
                operation: "jump cursor".to_string(),
                state: self.display_state.to_string()
            });
        };

        let table_data = self.get_table_data()?;
        table_data.is_valid_row_index(row)?;
        table_data.is_valid_col_index(col)?;
        self.display_state = DisplayState::DisplayTable(DisplayTableState {
            selected_cell: SelectedCell {
                table_name: selected_cell.table_name.clone(),
                row,
                col,
            },
            table_list: table_list.clone(),
        });

        Ok(())
    }

    /// Call from DisplayTable state
    /// Expand the row
    pub fn expand_row(&mut self) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(StrataError::InvalidOperationCall {
                operation: "expand row".to_string(),
                state: self.display_state.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;

        table_data.expand_row()
    }

    /// Call from DisplayTable display state
    /// Collapse the row
    pub fn collapse_row(&mut self, row: usize) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(StrataError::InvalidOperationCall {
                operation: "collapse row".to_string(),
                state: self.display_state.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;
        table_data.collapse_row(row)
    }

    pub fn expand_col(&mut self, col_name: &str) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(StrataError::InvalidOperationCall {
                operation: "expand col".to_string(),
                state: self.display_state.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;
        table_data.expand_col(col_name)
    }

    pub fn collapse_col(&mut self, col: usize) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(StrataError::InvalidOperationCall {
                operation: "collapse col".to_string(),
                state: self.display_state.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;
        table_data.collapse_col(col)
    }

    pub fn get_cell_value(&self) -> Result<&str> {
        match &self.display_state {
            DisplayState::DisplayTable(DisplayTableState { selected_cell, .. })
            | DisplayState::EditCell(EditCellState { selected_cell, .. }) => self
                .get_table_data()?
                .get_cell_value(selected_cell.row, selected_cell.col),
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "get cell value".to_string(),
                state: self.display_state.to_string()
            }),
        }
    }

    pub fn update_cell(&mut self, value: &str) -> Result<()> {
        let selected_cell;
        {
            let DisplayState::EditCell(EditCellState {
                selected_cell: sc, ..
            }) = &self.display_state
            else {
                bail!(StrataError::InvalidOperationCall {
                    operation: "update cell".to_string(),
                    state: self.display_state.to_string()
                });
            };

            selected_cell = sc.clone();
        }

        self.get_table_data_mut()?
            .update_cell(selected_cell.row, selected_cell.col, value)
    }

    fn get_table_data_mut(&mut self) -> Result<&mut TableData> {
        let table_name = self
            .get_selected_table_name()
            .ok_or_eyre(StrataError::NoTableSelected)?
            .clone();
        self.table_map
            .get_mut(&table_name)
            .ok_or_eyre(StrataError::TableNotFound(table_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use eyre::Context;

    use super::*;

    fn setup_select_table_app() -> App {
        let mut app = App::new();
        // setup table name
        let table_name_1 = TableName::from("table1").unwrap();
        let table_name_2 = TableName::from("table2").unwrap();
        // create 2x2 table
        let mut table_data = TableData::new().unwrap();
        table_data.expand_row().unwrap();
        table_data.expand_col("header").unwrap();
        table_data.update_cell(0, 0, "value0-0").unwrap();
        table_data.update_cell(1, 0, "value1-0").unwrap();
        table_data.update_cell(0, 1, "value0-1").unwrap();
        table_data.update_cell(1, 1, "value1-1").unwrap();
        app.table_map
            .insert(table_name_1.clone(), table_data.clone());
        app.table_map.insert(table_name_2.clone(), table_data);
        // set display state to DisplayTable
        app.display_state = DisplayState::SelectTable(SelectTableState {
            selected_cell: None,
            table_list: vec![table_name_1, table_name_2],
            cursor: 0,
        });

        app
    }

    fn setup_display_table_app() -> App {
        let mut app = App::new();
        // setup table name
        let table_name_1 = TableName::from("table1").unwrap();
        let table_name_2 = TableName::from("table2").unwrap();
        // create 2x2 table
        let mut table_data = TableData::new().unwrap();
        table_data.expand_row().unwrap();
        table_data.expand_col("header").unwrap();
        table_data.update_cell(0, 0, "value0-0").unwrap();
        table_data.update_cell(1, 0, "value1-0").unwrap();
        table_data.update_cell(0, 1, "value0-1").unwrap();
        table_data.update_cell(1, 1, "value1-1").unwrap();
        app.table_map
            .insert(table_name_1.clone(), table_data.clone());
        app.table_map.insert(table_name_2.clone(), table_data);
        // set display state to DisplayTable
        app.display_state = DisplayState::DisplayTable(DisplayTableState {
            selected_cell: SelectedCell::new(table_name_1.clone()),
            table_list: vec![table_name_1, table_name_2],
        });

        app
    }

    #[test]
    fn test_add_table() {
        let mut app = App::new();
        // add tables
        app.set_state_add_table().unwrap();
        app.add_table("table1").unwrap();
        app.set_state_select_table().unwrap();
        app.set_state_add_table().unwrap();
        app.add_table("table2").unwrap();

        assert_eq!(app.get_all_table_names().len(), 2);
        assert!(app
            .get_all_table_names()
            .contains(&&TableName::from("table1").unwrap()));
        assert!(app
            .get_all_table_names()
            .contains(&&TableName::from("table2").unwrap()));
    }

    #[test]
    fn test_select_table() {
        let mut app = setup_select_table_app();

        app.select_table().unwrap();
        assert_eq!(
            app.get_selected_table_name().map(|tn| tn.as_str()),
            Some("table1")
        );
    }

    #[test]
    fn test_move_table_selector() {
        let mut app = setup_select_table_app();

        app.down_table_selector().unwrap();
        assert_eq!(
            app.display_state,
            DisplayState::SelectTable(SelectTableState {
                selected_cell: None,
                table_list: vec![
                    TableName::from("table1").unwrap(),
                    TableName::from("table2").unwrap()
                ],
                cursor: 1,
            })
        );
        app.select_table().unwrap();
        assert_eq!(
            app.get_selected_table_name().map(|tn| tn.as_str()),
            Some("table2")
        );

        app.set_state_select_table().unwrap();
        app.up_table_selector().unwrap();
        app.select_table().unwrap();
        assert_eq!(
            app.get_selected_table_name().map(|tn| tn.as_str()),
            Some("table1")
        );
    }

    #[test]
    fn test_move_cell_selector() {
        let mut app = setup_display_table_app();

        // (0,0)
        assert_eq!(app.get_cell_value().unwrap(), "value0-0");
        // (1,0)
        app.move_cell_selector(1, 0)
            .with_context(|| {
                format!(
                    "move_cursor failed: table:{:?} selected_cell:{:?}",
                    app.get_table_data(),
                    app.get_selected_cell()
                )
            })
            .unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        // (1,1)
        app.move_cell_selector(0, 1)
            .with_context(|| format!("move_cursor failed: {:?}", app.get_selected_cell()))
            .unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1-1");
        // (0,1)
        app.move_cell_selector(-1, 0)
            .with_context(|| format!("move_cursor failed: {:?}", app.get_selected_cell()))
            .unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
    }

    #[test]
    fn test_jump_cursor() {
        let mut app = setup_display_table_app();

        app.jump_cell_selector(1, 0).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        app.jump_cell_selector(0, 1).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
    }
}
