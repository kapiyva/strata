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
    /// Setup a new App as SelectTable state
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

    /// Call from SelectTable state
    /// Change the display state to AddTable state
    pub fn set_state_add_table(&mut self) -> Result<()> {
        match &self.display_state {
            DisplayState::SelectTable(SelectTableState { selected_cell }) => {
                self.display_state = DisplayState::AddTable(AddTableState {
                    selected_cell: selected_cell.clone(),
                });
                Ok(())
            }
            _ => bail!(
                "Cannot change add table state from current state: {:?}",
                self.display_state
            ),
        }
    }

    /// Call from AddTable or DisplayTable state
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
            _ => bail!(
                "Cannot change select table state from current state: {:?}",
                self.display_state
            ),
        }
    }

    /// Call from DisplayTable state
    /// Change the display state to EditCell
    pub fn set_state_edit_cell(&mut self) -> Result<()> {
        // Only allow changing to EditCell state from DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState { selected_cell }) = &self.display_state
        else {
            bail!(
                "Cannot change edit cell state from current state: {:?}",
                self.display_state
            );
        };
        self.display_state = DisplayState::EditCell(EditCellState {
            selected_cell: selected_cell.clone(),
        });

        Ok(())
    }

    /// Call from AddTable state
    /// Add new table and change to DisplayTable state
    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        // Only allow adding table in AddTable state
        let DisplayState::AddTable(_) = self.display_state else {
            bail!(
                "Cannot add table in current state: {:?}",
                self.display_state
            );
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

    /// Call from SelectTable state
    /// Select table and change to DisplayTable state
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
            _ => bail!(
                "Cannot select table in current state: {:?}",
                self.display_state
            ),
        };
        Ok(())
    }

    /// Call from DisplayTable display state
    /// Move the cursor in the table
    pub fn move_cursor(&mut self, row: isize, col: isize) -> Result<()> {
        let selected_cell;
        {
            // Only allow moving cursor in DisplayTable state
            let DisplayState::DisplayTable(DisplayTableState { selected_cell: sc }) =
                &self.display_state
            else {
                bail!(
                    "Cannot move cursor in current state: {:?}",
                    self.display_state
                );
            };

            selected_cell = sc.clone();
        }

        let table_data = self.get_table_data_mut()?;
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

    /// Call from DisplayTable state
    /// Expand the row
    pub fn expand_row(&mut self) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(
                "Cannot expand row in current state: {:?}",
                self.display_state
            );
        };

        let table_data = self.get_table_data_mut()?;

        table_data.expand_row()
    }

    /// Call from DisplayTable display state
    /// Collapse the row
    pub fn collapse_row(&mut self, row: usize) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(
                "Cannot collapse row in current state: {:?}",
                self.display_state
            );
        };

        let table_data = self.get_table_data_mut()?;
        table_data.collapse_row(row)
    }

    pub fn expand_col(&mut self, col_name: &str) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(
                "Cannot expand col in current state: {:?}",
                self.display_state
            );
        };

        let table_data = self.get_table_data_mut()?;
        table_data.expand_col(col_name)
    }

    pub fn collapse_col(&mut self, col: usize) -> Result<()> {
        let DisplayState::DisplayTable(_) = &self.display_state else {
            bail!(
                "Cannot collapse col in current state: {:?}",
                self.display_state
            );
        };

        let table_data = self.get_table_data_mut()?;
        table_data.collapse_col(col)
    }

    pub fn jump_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        // Only allow jumping cursor in DisplayTable state
        let DisplayState::DisplayTable(DisplayTableState { selected_cell }) = &self.display_state
        else {
            bail!(
                "Cannot jump cursor in current state: {:?}",
                self.display_state
            );
        };

        let table_data = self.get_table_data().ok_or_eyre("table not found")?;
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
            | DisplayState::EditCell(EditCellState { selected_cell }) => self
                .get_table_data()
                .ok_or_eyre("table not found")?
                .get_cell(selected_cell.row, selected_cell.col)
                .map(|s| s.to_string()),
            _ => bail!(
                "Cannot get cell value in current state: {:?}",
                self.display_state
            ),
        }
    }

    pub fn update_cell(&mut self, value: &str) -> Result<()> {
        let selected_cell;
        {
            let DisplayState::EditCell(EditCellState { selected_cell: sc }) = &self.display_state
            else {
                bail!(
                    "Cannot update cell value in current state: {:?}",
                    self.display_state
                );
            };

            selected_cell = sc.clone();
        }

        self.get_table_data_mut()?
            .update_cell(selected_cell.row, selected_cell.col, value)
    }

    fn get_table_data_mut(&mut self) -> Result<&mut TableData> {
        let table_name = self
            .get_selected_table_name()
            .ok_or_eyre("No table selected")?
            .clone();
        self.table_map
            .get_mut(&table_name)
            .ok_or_eyre(format!("Table not found: {}", table_name.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use eyre::Context;

    use super::*;

    fn setup_display_table_app() -> App {
        let mut app = App::new();
        // setup table name
        let table_name = TableName::from("table1").unwrap();
        // create 2x2 table
        let mut table_data = TableData::new().unwrap();
        table_data.expand_row().unwrap();
        table_data.expand_col("header").unwrap();
        table_data.update_cell(0, 0, "value0-0").unwrap();
        table_data.update_cell(1, 0, "value1-0").unwrap();
        table_data.update_cell(0, 1, "value0-1").unwrap();
        table_data.update_cell(1, 1, "value1-1").unwrap();
        app.table_map.insert(table_name.clone(), table_data);
        // set display state to DisplayTable
        app.display_state = DisplayState::DisplayTable(DisplayTableState {
            selected_cell: SelectedCell::new(table_name),
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
        let mut app = App::new();
        // add table1
        app.set_state_add_table().unwrap();
        app.add_table("table1").unwrap();
        // add table2
        app.set_state_select_table().unwrap();
        app.set_state_add_table().unwrap();
        app.add_table("table2").unwrap();

        app.set_state_select_table().unwrap();
        app.select_table("table1").unwrap();
        assert_eq!(
            app.get_selected_table_name(),
            Some(&TableName::from("table1").unwrap())
        );

        app.set_state_select_table().unwrap();
        app.select_table("table2").unwrap();
        assert_eq!(
            app.get_selected_table_name(),
            Some(&TableName::from("table2").unwrap())
        );
    }

    #[test]
    fn test_move_cursor() {
        let mut app = setup_display_table_app();

        // (0,0)
        assert_eq!(app.get_cell_value().unwrap(), "value0-0");
        // (1,0)
        app.move_cursor(1, 0)
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
        app.move_cursor(0, 1)
            .with_context(|| format!("move_cursor failed: {:?}", app.get_selected_cell()))
            .unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1-1");
        // (0,1)
        app.move_cursor(-1, 0)
            .with_context(|| format!("move_cursor failed: {:?}", app.get_selected_cell()))
            .unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
    }

    #[test]
    fn test_jump_cursor() {
        let mut app = setup_display_table_app();

        app.jump_cursor(1, 0).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        app.jump_cursor(0, 1).unwrap();
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
    }
}
