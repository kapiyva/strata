pub mod state;

use std::collections::HashMap;

use color_eyre::eyre::Result;
use eyre::{bail, OptionExt};
use state::*;

use crate::error::StrataError;

use super::table::{TableData, TableName};

#[derive(Default)]
pub struct App {
    display_mode: DisplayMode,
    table_list: Vec<TableName>,
    table_map: HashMap<TableName, TableData>,
    table_selector_index: Option<usize>,
    // selected_cell: Option<SelectedCell>,
    cell_selector_index: Option<(usize, usize)>,
    exiting: bool,
}

impl App {
    /// Setup a new App as SelectTable state
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_exit(&self) -> bool {
        self.exiting
    }

    pub fn set_exit(&mut self, exit: bool) -> Result<()> {
        self.exiting = exit;
        Ok(())
    }

    pub fn get_display_mode(&self) -> &DisplayMode {
        &self.display_mode
    }

    pub fn get_all_table_names(&self) -> &Vec<TableName> {
        &self.table_list
    }

    pub fn get_selected_table_name(&self) -> Option<&TableName> {
        self.table_list.get(self.table_selector_index?)
    }

    pub fn get_selected_cell(&self) -> Option<(usize, usize)> {
        self.cell_selector_index
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
        match &mut self.display_mode {
            DisplayMode::SelectTable => {
                self.display_mode = DisplayMode::AddTable;
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "set state AddTable".to_string(),
                mode: self.display_mode.to_string()
            }),
        }
    }

    /// Call from AddTable or DisplayTable state
    /// Change the display state to SelectTable
    pub fn set_state_select_table(&mut self) -> Result<()> {
        match &mut self.display_mode {
            DisplayMode::AddTable | DisplayMode::SelectCell => {
                self.display_mode = DisplayMode::SelectTable;
                if self.table_list.is_empty() {
                    self.table_selector_index = None;
                } else if self.table_selector_index.is_none() {
                    self.table_selector_index = Some(0);
                }
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "set state SelectTable".to_string(),
                mode: self.display_mode.to_string()
            }),
        }
    }

    // Call from EditCell state
    // Cancel edit and change the display state to SelectCell
    pub fn set_state_select_cell(&mut self) -> Result<()> {
        match &mut self.display_mode {
            DisplayMode::EditCell => {
                self.display_mode = DisplayMode::SelectCell;
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "set state SelectCell".to_string(),
                mode: self.display_mode.to_string()
            }),
        }
    }

    /// Call from DisplayTable state
    /// Change the display state to EditCell
    pub fn set_state_edit_cell(&mut self) -> Result<()> {
        // Only allow changing to EditCell state from DisplayTable state
        let DisplayMode::SelectCell = &mut self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "set state EditCell".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        self.display_mode = DisplayMode::EditCell;

        Ok(())
    }

    /// Call from AddTable state
    /// Add new table and change to DisplayTable state
    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        // Only allow adding table in AddTable state
        let DisplayMode::AddTable = &mut self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "add table".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let table_name = TableName::from(table_name_str)?;
        if self.table_map.contains_key(&table_name) {
            bail!(StrataError::TableNameDuplicate(table_name_str.to_string()));
        }

        self.table_map.insert(table_name.clone(), TableData::new()?);
        self.table_list.push(table_name.clone());
        self.display_mode = DisplayMode::SelectCell;
        self.table_selector_index = Some(self.table_list.len() - 1);
        self.cell_selector_index = Some((0, 0));
        Ok(())
    }

    /// Call from SelectTable state
    /// Move the table selector down
    pub fn down_table_selector(&mut self) -> Result<()> {
        if self.table_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }
        let Some(index) = &mut self.table_selector_index else {
            bail!(StrataError::NoTableSelected);
        };

        // let new_index = *index + 1;

        match &mut self.display_mode {
            DisplayMode::SelectTable => {
                *index = (*index + 1).min(self.table_list.len() - 1);
                // if new_index < self.table_list.len() - 1 {
                //     *index = new_index;
                // } else {
                //     *index = self.table_list.len() - 1;
                // }
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "down table selector".to_string(),
                mode: self.display_mode.to_string()
            }),
        }
    }

    /// Call from SelectTable state
    /// Move the table selector up
    pub fn up_table_selector(&mut self) -> Result<()> {
        let Some(index) = &mut self.table_selector_index else {
            bail!(StrataError::NoTableSelected);
        };

        match &mut self.display_mode {
            DisplayMode::SelectTable => {
                if *index > 0 {
                    *index -= 1;
                } else {
                    *index = 0;
                }
                Ok(())
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "up table selector".to_string(),
                mode: self.display_mode.to_string()
            }),
        }
    }

    /// Call from SelectTable state
    /// Select table and change to DisplayTable state
    pub fn select_table(&mut self) -> Result<()> {
        let DisplayMode::SelectTable = &mut self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "select table".to_string(),
                mode: self.display_mode.to_string()
            });
        };
        if self.table_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }

        self.display_mode = DisplayMode::SelectCell;
        self.cell_selector_index = Some((0, 0));
        Ok(())
    }

    /// Call from SelectTable state
    /// Remove the selected table
    pub fn remove_table(&mut self) -> Result<()> {
        let DisplayMode::SelectTable = &mut self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "remove table".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        if self.table_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }
        let target_table_name = self
            .get_selected_table_name()
            .ok_or_eyre(StrataError::NoTableSelected)?
            .clone();

        self.table_map.remove(&target_table_name);
        self.table_list.retain(|tn| tn != &target_table_name);
        self.display_mode = DisplayMode::SelectTable;
        self.table_selector_index = Some(0);
        self.cell_selector_index = None;
        Ok(())
    }

    /// Call from DisplayTable display state
    /// Move the cursor in the table
    pub fn move_cell_selector(&mut self, row_move: isize, col_move: isize) -> Result<()> {
        // Only allow moving cursor in DisplayTable state
        let DisplayMode::SelectCell = &self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "move cursor".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let (selected_row, selected_col) = self
            .cell_selector_index
            .ok_or_eyre(StrataError::NoCellSelected)?;
        let (max_row, max_col) = {
            let table_data = self.get_table_data()?;
            (
                table_data.get_max_row_index(),
                table_data.get_max_col_index(),
            )
        };
        let new_row = match row_move {
            0 => selected_row,
            _ if row_move < 0 => {
                let row_move_abs = row_move.unsigned_abs();
                if selected_row < row_move_abs {
                    0
                } else {
                    (selected_row - row_move_abs).max(0)
                }
            }
            _ if row_move > 0 => (selected_row + (row_move as usize)).min(max_row),
            _ => unreachable!(),
        };
        let new_col = match col_move {
            0 => selected_col,
            _ if col_move < 0 => {
                let col_move_abs = col_move.unsigned_abs();
                if selected_col < col_move_abs {
                    0
                } else {
                    (selected_col - col_move_abs).max(0)
                }
            }
            _ if col_move > 0 => (selected_col + (col_move as usize)).min(max_col),
            _ => unreachable!(),
        };

        self.cell_selector_index = Some((new_row, new_col));
        Ok(())
    }

    /// Call from DisplayTable state
    /// Jump the cursor to the specified cell
    pub fn jump_cell_selector(&mut self, row: usize, col: usize) -> Result<()> {
        // Only allow jumping cursor in DisplayTable state
        let DisplayMode::SelectCell = &self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "jump cursor".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let table_data = self.get_table_data()?;
        table_data.is_valid_row_index(row)?;
        table_data.is_valid_col_index(col)?;

        self.cell_selector_index = Some((row, col));
        Ok(())
    }

    /// Call from DisplayTable state
    /// Expand the row
    pub fn expand_row(&mut self) -> Result<()> {
        let DisplayMode::SelectCell = &self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "expand row".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;

        table_data.expand_row()
    }

    /// Call from DisplayTable display state
    /// Collapse the row
    pub fn collapse_row(&mut self, row: usize) -> Result<()> {
        let DisplayMode::SelectCell = &self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "collapse row".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;
        table_data.collapse_row(row)
    }

    pub fn expand_col(&mut self, col_name: &str) -> Result<()> {
        let DisplayMode::SelectCell = &self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "expand col".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;
        table_data.expand_col(col_name)
    }

    pub fn collapse_col(&mut self, col: usize) -> Result<()> {
        let DisplayMode::SelectCell = &self.display_mode else {
            bail!(StrataError::InvalidOperationCall {
                operation: "collapse col".to_string(),
                mode: self.display_mode.to_string()
            });
        };

        let table_data = self.get_table_data_mut()?;
        table_data.collapse_col(col)
    }

    pub fn get_cell_value(&self) -> Result<&str> {
        let (row, col) = self
            .cell_selector_index
            .ok_or_eyre(StrataError::NoCellSelected)?;

        match &self.display_mode {
            DisplayMode::SelectCell | DisplayMode::EditCell => {
                self.get_table_data()?.get_cell_value(row, col)
            }
            _ => bail!(StrataError::InvalidOperationCall {
                operation: "get cell value".to_string(),
                mode: self.display_mode.to_string()
            }),
        }
    }

    pub fn update_cell_value(&mut self, value: &str) -> Result<()> {
        let (row, col) = self
            .cell_selector_index
            .ok_or_eyre(StrataError::NoCellSelected)?;

        self.display_mode = DisplayMode::SelectCell;
        self.get_table_data_mut()?.update_cell(row, col, value)
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
        // setup app state
        app.display_mode = DisplayMode::SelectTable;
        app.table_list = vec![table_name_1.clone(), table_name_2.clone()];
        app.table_map
            .insert(table_name_1.clone(), table_data.clone());
        app.table_map.insert(table_name_2.clone(), table_data);
        app.table_selector_index = Some(0);
        app.cell_selector_index = None;

        app
    }

    fn setup_select_cell_app() -> App {
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
        // setup app state
        app.display_mode = DisplayMode::SelectCell;
        app.table_list = vec![table_name_1.clone(), table_name_2.clone()];
        app.table_map
            .insert(table_name_1.clone(), table_data.clone());
        app.table_map.insert(table_name_2.clone(), table_data);
        app.table_selector_index = Some(0);
        app.cell_selector_index = Some((0, 0));

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
    fn test_remove_table() {
        let mut app = setup_select_table_app();

        app.remove_table().unwrap();
        assert_eq!(app.get_all_table_names().len(), 1);
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

        // check down table selector
        app.down_table_selector().unwrap();
        assert_eq!(app.table_selector_index, Some(1),);
        // check out of bound
        app.down_table_selector().unwrap();
        assert_eq!(app.table_selector_index, Some(1),);

        // check up table selector
        app.up_table_selector().unwrap();
        assert_eq!(app.table_selector_index, Some(0),);
        // check out of bound
        app.up_table_selector().unwrap();
        assert_eq!(app.table_selector_index, Some(0),);
    }

    #[test]
    fn test_move_cell_selector() {
        // (0,0)
        let mut app = setup_select_cell_app();

        // (1,0)
        app.move_cell_selector(1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        // check out of bound
        app.move_cell_selector(1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");

        // (1,1)
        app.move_cell_selector(0, 1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-1");
        // check out of bound
        app.move_cell_selector(0, 1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-1");

        // (0,1)
        app.move_cell_selector(-1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
        // check out of bound
        app.move_cell_selector(-1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");

        // (0,0)
        app.move_cell_selector(0, -1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-0");
        // check out of bound
        app.move_cell_selector(0, -1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-0");
    }

    #[test]
    fn test_jump_cell_selector() {
        let mut app = setup_select_cell_app();

        app.jump_cell_selector(1, 0).expect("Failed to jump (1,0)");
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        app.jump_cell_selector(0, 1).expect("Failed to jump (0,1)");
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
        // check jump to out of bound
        if let Ok(_) = app.jump_cell_selector(2, 1) {
            panic!("jump_cursor should fail");
        }
        if let Ok(_) = app.jump_cell_selector(1, 2) {
            panic!("jump_cursor should fail");
        }
    }
}
