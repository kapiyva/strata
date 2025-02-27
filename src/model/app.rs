pub mod state;

use color_eyre::eyre::Result;
use eyre::{bail, OptionExt};
use ratatui::widgets::TableState;
use state::DisplayFocus;

use crate::error::StrataError;

use super::table::{TableData, TableName};

pub struct AppCommand {
    command_name: String,
    function: Box<dyn FnOnce(&mut App) -> Result<()>>,
}

impl AppCommand {
    pub fn new(
        command_name: &str,
        function: Box<dyn FnOnce(&mut App) -> Result<()>>,
    ) -> AppCommand {
        AppCommand {
            command_name: command_name.to_string(),
            function,
        }
    }
}

#[derive(Default)]
pub struct App {
    display_focus: DisplayFocus,
    table_name_list: Vec<TableName>,
    table_data_list: Vec<TableData>,
    table_selector: Option<usize>,
    table_state: TableState,
    input: String,
    command: Option<AppCommand>,
    error_message: Vec<String>,
}

impl App {
    /// Setup a new App as SelectTable mode
    pub fn new() -> Self {
        Self::default()
    }

    pub fn focus_last(&mut self) -> Result<()> {
        match &self.display_focus {
            DisplayFocus::TableList => Ok(()),
            DisplayFocus::TableView => Ok(self.focus_table_list()),
            DisplayFocus::Command(_) | DisplayFocus::Error(_) | DisplayFocus::Exit(_) => {
                match DisplayFocus::last_focus(&self.display_focus) {
                    DisplayFocus::TableList => Ok(self.focus_table_list()),
                    DisplayFocus::TableView => self.focus_table_view(),
                    _ => bail!(StrataError::InvalidOperationCall {
                        operation: "cancel".to_string(),
                        mode: self.display_focus.to_string(),
                    }),
                }
            }
        }
    }

    pub fn get_display_focus(&self) -> &DisplayFocus {
        &self.display_focus
    }

    pub fn get_table_name_list(&self) -> &Vec<TableName> {
        &self.table_name_list
    }

    pub fn get_selected_table_name(&self) -> Option<&TableName> {
        self.table_name_list.get(self.table_selector?)
    }

    pub fn update_table_name(&mut self, new_name: &str) -> Result<()> {
        let table_selector = self
            .table_selector
            .ok_or_eyre(StrataError::NoTableSelected)?;

        let new_table_name = TableName::from(new_name)?;
        if self.table_name_list.contains(&new_table_name) {
            bail!(StrataError::TableNameDuplicate(new_name.to_string()));
        }

        self.table_name_list[table_selector] = new_table_name;
        Ok(())
    }

    pub fn get_selected_table_data(&self) -> Result<&TableData> {
        let table_selector = self
            .table_selector
            .ok_or_eyre(StrataError::NoTableSelected)?;

        self.table_data_list
            .get(table_selector)
            .ok_or_eyre(StrataError::TableNotFound(table_selector.to_string()))
    }

    pub fn get_table_selector(&self) -> Option<usize> {
        self.table_selector
    }

    pub fn get_table_state(&self) -> &TableState {
        &self.table_state
    }

    pub fn get_selected_cell(&self) -> Option<(usize, usize)> {
        self.table_state.selected_cell()
    }

    pub fn get_user_input(&self) -> &str {
        &self.input
    }

    pub fn push_user_input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_user_input(&mut self) {
        self.input.pop();
    }

    pub fn clear_user_input(&mut self) {
        self.input.clear();
    }

    pub fn get_command_name(&self) -> Option<&str> {
        self.command.as_ref().map(|c| c.command_name.as_str())
    }

    pub fn clear_command(&mut self) {
        self.command = None;
    }

    pub fn execute_command(&mut self) -> Result<()> {
        let command = self
            .command
            .take()
            .ok_or_eyre(StrataError::CommandNotFound)?;
        (command.function)(self)?;
        self.command = None;
        Ok(())
    }

    pub fn get_error_message(&self) -> &Vec<String> {
        &self.error_message
    }

    pub fn push_error_message(&mut self, message: String) {
        self.error_message.push(message);
    }

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn focus_table_list(&mut self) {
        self.display_focus = DisplayFocus::TableList;
        if self.table_name_list.is_empty() {
            self.table_selector = None;
        } else if self.table_selector.is_none() {
            self.table_selector = Some(0);
        }
    }

    pub fn focus_table_view(&mut self) -> Result<()> {
        if self.table_name_list.is_empty() | self.table_data_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        } else if self.table_selector.is_none() {
            bail!(StrataError::NoTableSelected);
        }

        self.display_focus = DisplayFocus::TableView;
        if self.table_state.selected_cell().is_none() {
            self.table_state.select_cell(Some((0, 0)));
        }
        Ok(())
    }

    pub fn focus_table_view_by_name(&mut self, table_name: &str) -> Result<()> {
        if self.table_name_list.is_empty() | self.table_data_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }

        let table_name = TableName::from(table_name)?;

        self.table_selector = Some(
            self.table_name_list
                .iter()
                .position(|tn| *tn == table_name)
                .ok_or_eyre(StrataError::TableNotFound(table_name.to_string()))?,
        );
        self.display_focus = DisplayFocus::TableView;
        self.table_state = TableState::default().with_selected_cell(Some((0, 0)));
        Ok(())
    }

    pub fn focus_command(&mut self, command: AppCommand) {
        self.command = Some(command);
        self.display_focus = DisplayFocus::Command(Box::new(self.display_focus.clone()));
    }

    pub fn focus_error(&mut self) {
        if !self.error_message.is_empty() {
            self.display_focus = DisplayFocus::Error(Box::new(self.display_focus.clone()));
        }
    }

    pub fn focus_exit(&mut self) {
        self.display_focus = DisplayFocus::Exit(Box::new(self.display_focus.clone()));
    }

    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        let table_name = TableName::from(table_name_str)?;
        if self.table_name_list.contains(&table_name) {
            bail!(StrataError::TableNameDuplicate(table_name_str.to_string()));
        }

        self.table_data_list.push(TableData::new());
        self.table_name_list.push(table_name.clone());
        Ok(())
    }

    /// Call from SelectTable mode
    /// Move the table selector down
    pub fn down_table_selector(&mut self) -> Result<()> {
        if self.table_name_list.is_empty() {
            self.table_selector = None;
            bail!(StrataError::NoTableAdded);
        }
        let Some(index) = &mut self.table_selector else {
            self.table_selector = Some(0);
            return Ok(());
        };

        *index = (*index + 1).min(self.table_name_list.len() - 1);
        Ok(())
    }

    /// Move the table selector up
    pub fn up_table_selector(&mut self) -> Result<()> {
        if self.table_name_list.is_empty() {
            self.table_selector = None;
            bail!(StrataError::NoTableAdded);
        }
        let Some(index) = &mut self.table_selector else {
            self.table_selector = Some(0);
            return Ok(());
        };

        *index = (*index).saturating_sub(1);
        Ok(())
    }

    /// Select table and focus to TableView
    pub fn select_table(&mut self) -> Result<()> {
        if self.table_name_list.is_empty() || self.table_data_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }

        self.display_focus = DisplayFocus::TableView;
        self.table_state = TableState::default();
        self.table_state.select_cell(Some((0, 0)));
        Ok(())
    }

    /// Call from SelectTable mode
    /// Remove the selected table
    pub fn remove_table(&mut self) -> Result<()> {
        if self.table_name_list.is_empty() || self.table_data_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }
        let target_table_index = self
            .table_selector
            .ok_or_eyre(StrataError::NoTableSelected)?;

        self.table_data_list.remove(target_table_index);
        self.table_name_list.remove(target_table_index);
        self.display_focus = DisplayFocus::TableList;
        self.table_selector = Some(0);
        self.table_state.select_cell(None);
        Ok(())
    }

    /// Call from SelectCell display mode
    /// Move the cursor in the table
    pub fn move_cell_selector(&mut self, row_move: isize, col_move: isize) -> Result<()> {
        let DisplayFocus::TableView = &self.display_focus else {
            bail!(StrataError::InvalidOperationCall {
                operation: "move cursor".to_string(),
                mode: self.display_focus.to_string()
            });
        };

        let (max_row, max_col) = {
            let table_data = self.get_selected_table_data()?;
            (
                table_data.get_max_row_index(),
                table_data.get_max_col_index(),
            )
        };
        let (selected_row, selected_col) = self
            .table_state
            .selected_cell()
            .ok_or_eyre(StrataError::NoCellSelected)?;

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

        self.table_state.select_cell(Some((new_row, new_col)));
        Ok(())
    }

    /// Call from SelectCell mode
    /// Jump the cursor to the specified cell
    pub fn jump_cell_selector(&mut self, row: usize, col: usize) -> Result<()> {
        let DisplayFocus::TableView = &self.display_focus else {
            bail!(StrataError::InvalidOperationCall {
                operation: "jump cursor".to_string(),
                mode: self.display_focus.to_string()
            });
        };

        let table_data = self.get_selected_table_data()?;
        table_data.is_valid_row_index(row)?;
        table_data.is_valid_col_index(col)?;

        self.table_state.select_cell(Some((row.clone(), col)));
        Ok(())
    }

    /// Expand the row
    pub fn expand_row(&mut self) -> Result<()> {
        self.get_table_data_mut()?.expand_row()
    }

    /// Collapse the row
    pub fn collapse_row(&mut self, row: usize) -> Result<()> {
        self.get_table_data_mut()?.collapse_row(row)
    }

    /// Expand the column
    pub fn expand_col(&mut self) -> Result<()> {
        let table_data = self.get_table_data_mut()?;
        let header = format!("header{}", table_data.get_max_col_index() + 1);

        table_data.expand_col(&header)
    }

    /// Collapse the column
    pub fn collapse_col(&mut self, col: usize) -> Result<()> {
        self.get_table_data_mut()?.collapse_col(col)
    }

    pub fn update_header(&mut self, value: &str) -> Result<()> {
        let col = self
            .table_state
            .selected_column()
            .ok_or_eyre(StrataError::NoCellSelected)?;

        self.display_focus = DisplayFocus::TableView;
        self.get_table_data_mut()?.update_header(col, value)
    }

    pub fn get_cell_value(&self) -> Result<&str> {
        let (row, col) = self
            .table_state
            .selected_cell()
            .ok_or_eyre(StrataError::NoCellSelected)?;

        self.get_selected_table_data()?.get_cell_value(row, col)
    }

    pub fn update_cell_value(&mut self, value: &str) -> Result<()> {
        let (row, col) = self
            .table_state
            .selected_cell()
            .ok_or_eyre(StrataError::NoCellSelected)?;

        self.display_focus = DisplayFocus::TableView;
        self.get_table_data_mut()?.update_cell(row, col, value)
    }

    fn get_table_data_mut(&mut self) -> Result<&mut TableData> {
        let index = self
            .table_selector
            .ok_or_eyre(StrataError::NoTableSelected)?;
        self.table_data_list
            .get_mut(index)
            .ok_or_eyre(StrataError::TableNotFound(index.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_focus_table_list_app() -> App {
        let mut app = App::new();
        // setup table name
        let table_name_1 = TableName::from("table1").unwrap();
        let table_name_2 = TableName::from("table2").unwrap();
        // create 2x2 table
        let mut table_data = TableData::default();
        table_data.expand_row().unwrap();
        table_data.expand_col("header1").unwrap();
        table_data.update_cell(0, 0, "value0-0").unwrap();
        table_data.update_cell(1, 0, "value1-0").unwrap();
        table_data.update_cell(0, 1, "value0-1").unwrap();
        table_data.update_cell(1, 1, "value1-1").unwrap();
        // setup app state
        app.display_focus = DisplayFocus::TableList;
        app.table_name_list = vec![table_name_1.clone(), table_name_2.clone()];
        app.table_data_list.push(table_data.clone());
        app.table_data_list.push(table_data);
        app.table_selector = Some(0);
        app.table_state.select_cell(None);

        app
    }

    fn setup_focus_table_view_app() -> App {
        let mut app = App::new();
        // setup table name
        let table_name_1 = TableName::from("table1").unwrap();
        let table_name_2 = TableName::from("table2").unwrap();
        // create 2x2 table
        let mut table_data = TableData::default();
        table_data.expand_row().unwrap();
        table_data.expand_col("header1").unwrap();
        table_data.update_cell(0, 0, "value0-0").unwrap();
        table_data.update_cell(1, 0, "value1-0").unwrap();
        table_data.update_cell(0, 1, "value0-1").unwrap();
        table_data.update_cell(1, 1, "value1-1").unwrap();
        // setup app state
        app.display_focus = DisplayFocus::TableView;
        app.table_name_list = vec![table_name_1.clone(), table_name_2.clone()];
        app.table_data_list.push(table_data.clone());
        app.table_data_list.push(table_data);
        app.table_selector = Some(0);
        app.table_state = {
            let mut ts = TableState::default();
            ts.select_cell(Some((0, 0)));
            ts
        };

        app
    }

    #[test]
    fn test_add_table() {
        let mut app = App::new();
        // add tables
        app.add_table("table1").unwrap();
        app.add_table("table2").unwrap();

        assert_eq!(app.get_table_name_list().len(), 2);
        assert!(app
            .get_table_name_list()
            .contains(&&TableName::from("table1").unwrap()));
        assert!(app
            .get_table_name_list()
            .contains(&&TableName::from("table2").unwrap()));
    }

    #[test]
    fn test_remove_table() {
        let mut app = setup_focus_table_list_app();

        app.remove_table().unwrap();
        assert_eq!(app.get_table_name_list().len(), 1);
        assert!(app
            .get_table_name_list()
            .contains(&&TableName::from("table2").unwrap()));
    }

    #[test]
    fn test_select_table() {
        let mut app = setup_focus_table_list_app();

        app.select_table().unwrap();
        assert_eq!(
            app.get_selected_table_name().map(|tn| tn.as_str()),
            Some("table1")
        );
    }

    #[test]
    fn test_move_table_selector() {
        let mut app = setup_focus_table_list_app();

        // check down table selector
        app.down_table_selector().unwrap();
        assert_eq!(app.table_selector, Some(1),);
        // check out of bound
        app.down_table_selector().unwrap();
        assert_eq!(app.table_selector, Some(1),);

        // check up table selector
        app.up_table_selector().unwrap();
        assert_eq!(app.table_selector, Some(0),);
        // check out of bound
        app.up_table_selector().unwrap();
        assert_eq!(app.table_selector, Some(0),);
    }

    #[test]
    fn test_move_cell_selector() {
        // (0,0)
        let mut app = setup_focus_table_view_app();

        // (1,0)
        app.move_cell_selector(1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        // check out of bound
        app.move_cell_selector(1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");

        // (1,1)
        app.move_cell_selector(0, 1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-1");
        // check out of bound
        app.move_cell_selector(0, 1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value1-1");

        // (0,1)
        app.move_cell_selector(-1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
        // check out of bound
        app.move_cell_selector(-1, 0).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");

        // (0,0)
        app.move_cell_selector(0, -1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-0");
        // check out of bound
        app.move_cell_selector(0, -1).expect(&format!(
            "move_cursor failed: {:?}",
            app.get_table_state().selected_cell()
        ));
        assert_eq!(app.get_cell_value().unwrap(), "value0-0");
    }

    #[test]
    fn test_jump_cell_selector() {
        let mut app = setup_focus_table_view_app();

        app.jump_cell_selector(1, 0).expect("Failed to jump (1,0)");
        assert_eq!(app.get_cell_value().unwrap(), "value1-0");
        app.jump_cell_selector(0, 1).expect("Failed to jump (0,1)");
        assert_eq!(app.get_cell_value().unwrap(), "value0-1");
        // check jump to out of bound
        if let Ok(_) = app.jump_cell_selector(2, 1) {
            panic!("jump_cursor should fail cell_selector",);
        }
        if let Ok(_) = app.jump_cell_selector(1, 2) {
            panic!("jump_cursor should fail");
        }
    }

    #[test]
    fn test_focus_table_view_by_name() {
        let mut app = setup_focus_table_list_app();

        app.focus_table_view_by_name("table2").unwrap();
        assert_eq!(app.get_display_focus().to_string(), "TableView");
        assert_eq!(
            app.get_selected_table_name().map(|tn| tn.as_str()),
            Some("table2")
        );
    }

    #[test]
    fn test_focus_command() {
        let mut app = setup_focus_table_list_app();
        let command = AppCommand::new("test", Box::new(|_| Ok(())));
        app.focus_command(command);

        assert_eq!(
            app.get_display_focus(),
            &DisplayFocus::Command(Box::new(DisplayFocus::TableList))
        );
        assert_eq!(app.get_command_name(), Some("test"));
        assert_eq!(
            DisplayFocus::last_focus(app.get_display_focus()),
            DisplayFocus::TableList
        );
        assert!(app.execute_command().is_ok());
    }
}
