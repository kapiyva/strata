pub mod base_component;
pub mod component;
pub mod display_focus;

use std::{ffi::OsStr, path::Path};

use color_eyre::eyre::Result;
use display_focus::DisplayFocus;
use eyre::{bail, OptionExt};

use crate::error::StrataError;

use component::{
    command::CommandPopup,
    error_popup::ErrorPopup,
    table_selector::{TableName, TableSelector, INITIAL_TABLE_NAME},
    table_view::TableView,
};

#[derive(Default)]
pub struct App {
    display_focus: DisplayFocus,
    table_selector: TableSelector,
    table_view_list: Vec<TableView>,
    command: Option<CommandPopup>,
    error_popup: ErrorPopup,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display_focus(&self) -> &DisplayFocus {
        &self.display_focus
    }

    pub fn table_selector(&self) -> &TableSelector {
        &self.table_selector
    }

    pub fn table_selector_mut(&mut self) -> &mut TableSelector {
        &mut self.table_selector
    }

    pub fn selected_table_view(&self) -> Result<&TableView> {
        let index = self
            .table_selector
            .selected_index()
            .ok_or_eyre(StrataError::NoTableSelected)?;

        self.table_view_list
            .get(index)
            .ok_or_eyre(StrataError::TableNotFound(index.to_string()))
    }

    pub fn selected_table_view_mut(&mut self) -> Result<&mut TableView> {
        let index = self
            .table_selector
            .selected_index()
            .ok_or_eyre(StrataError::NoTableSelected)?;
        self.table_view_list
            .get_mut(index)
            .ok_or_eyre(StrataError::TableNotFound(index.to_string()))
    }

    pub fn command(&self) -> Option<&CommandPopup> {
        self.command.as_ref()
    }

    pub fn command_mut(&mut self) -> Option<&mut CommandPopup> {
        self.command.as_mut()
    }

    pub fn command_name(&self) -> Option<&str> {
        self.command.as_ref().map(CommandPopup::command_name)
    }

    pub fn error_popup(&self) -> &ErrorPopup {
        &self.error_popup
    }

    pub fn error_popup_mut(&mut self) -> &mut ErrorPopup {
        &mut self.error_popup
    }

    pub fn focus_table_selector(&mut self) -> &mut Self {
        self.display_focus = DisplayFocus::TableSelector;
        self
    }

    pub fn focus_table_view(&mut self) -> Result<&mut Self> {
        if self.table_selector.is_empty() {
            bail!(StrataError::NoTableAdded);
        }

        self.display_focus = DisplayFocus::TableView;
        Ok(self)
    }

    pub fn focus_table_view_by_name(&mut self, table_name: &str) -> Result<&mut Self> {
        if self.table_selector.is_empty() {
            bail!(StrataError::NoTableAdded);
        }

        let table_name = TableName::from(table_name)?;

        self.table_selector_mut().select_by_name(&table_name)?;
        self.display_focus = DisplayFocus::TableView;
        Ok(self)
    }

    pub fn focus_command(&mut self, command: CommandPopup) -> &mut Self {
        self.command = Some(command);
        self.display_focus = DisplayFocus::Command(Box::new(self.display_focus.clone()));
        self
    }

    pub fn focus_error(&mut self) -> &mut Self {
        if !self.error_popup.is_empty() {
            self.display_focus = DisplayFocus::Error(Box::new(self.display_focus.clone()));
        }
        self
    }

    pub fn focus_exit(&mut self) -> &mut Self {
        self.display_focus = DisplayFocus::Exit(Box::new(self.display_focus.clone()));
        self
    }

    pub fn focus_last(&mut self) -> Result<&mut Self> {
        match &self.display_focus {
            DisplayFocus::TableSelector => Ok(self),
            DisplayFocus::TableView => Ok(self.focus_table_selector()),
            DisplayFocus::Command(_) | DisplayFocus::Error(_) | DisplayFocus::Exit(_) => {
                match DisplayFocus::last_focus(&self.display_focus) {
                    DisplayFocus::TableSelector => Ok(self.focus_table_selector()),
                    DisplayFocus::TableView => self.focus_table_view(),
                    _ => bail!(StrataError::InvalidOperationCall {
                        operation: "cancel".to_string(),
                        focus: self.display_focus.to_string(),
                    }),
                }
            }
        }
    }

    pub fn add_table(&mut self, table_name_str: &str) -> Result<()> {
        let table_name = TableName::from(table_name_str)?;

        self.table_selector.push_table(table_name)?;
        self.table_view_list.push(TableView::new());
        Ok(())
    }

    pub fn open_table(&mut self, file_path: &Path, has_header: bool) -> Result<()> {
        let table_name = file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .map_or(TableName::from(INITIAL_TABLE_NAME), TableName::from)?;
        let new_table = TableView::from_csv(file_path, has_header)?;

        self.table_selector.push_table(table_name)?;
        self.table_view_list.push(new_table);
        Ok(())
    }

    pub fn remove_table(&mut self) -> Result<()> {
        if self.table_selector.is_empty() || self.table_view_list.is_empty() {
            bail!(StrataError::NoTableAdded);
        }

        let index = self
            .table_selector
            .selected_index()
            .ok_or_eyre(StrataError::NoTableSelected)?;

        self.table_selector.remove_table(index)?;
        self.table_view_list.remove(index);
        self.display_focus = DisplayFocus::TableSelector;
        Ok(())
    }

    /// Execute the command and discard it
    pub fn execute_command(&mut self) -> Result<()> {
        self.command
            .take()
            .ok_or_eyre(StrataError::CommandNotFound)?
            .execute(self)
    }

    pub fn clear_command(&mut self) {
        self.command = None;
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
        let mut table_view = TableView::default();
        table_view
            .expand_row()
            .expand_col()
            .update_cell(0, 0, "value0-0")
            .and_then(|tv| tv.update_cell(1, 0, "value1-0"))
            .and_then(|tv| tv.update_cell(0, 1, "value0-1"))
            .and_then(|tv| tv.update_cell(1, 1, "value1-1"))
            .unwrap();
        // setup app state
        app.display_focus = DisplayFocus::TableSelector;
        app.table_selector = TableSelector::from(vec![table_name_1.clone(), table_name_2.clone()]);
        app.table_view_list.push(table_view.clone());
        app.table_view_list.push(table_view);

        app
    }

    #[test]
    fn test_add_table() {
        let mut app = App::new();
        // add tables
        app.add_table("table1").unwrap();
        app.add_table("table2").unwrap();

        assert_eq!(app.table_selector.get_table_list().len(), 2);
        assert!(app
            .table_selector
            .get_table_list()
            .contains(&&TableName::from("table1").unwrap()));
        assert!(app
            .table_selector
            .get_table_list()
            .contains(&&TableName::from("table2").unwrap()));
        assert_eq!(app.table_view_list.len(), 2);
    }

    #[test]
    fn test_remove_table() {
        let mut app = setup_focus_table_list_app();

        app.remove_table().unwrap();
        assert_eq!(app.table_selector.get_table_list().len(), 1);
        assert!(app
            .table_selector
            .get_table_list()
            .contains(&&TableName::from("table2").unwrap()));
    }

    #[test]
    fn test_focus_table_view_by_name() {
        let mut app = setup_focus_table_list_app();

        app.focus_table_view_by_name("table2").unwrap();
        assert_eq!(app.display_focus().to_string(), "TableView");
        assert_eq!(app.table_selector.selected_index(), Some(1));
    }

    #[test]
    fn test_focus_command() {
        let mut app = setup_focus_table_list_app();
        let command = CommandPopup::new("test", "", Box::new(|_, _| Ok(())));
        app.focus_command(command);

        assert_eq!(
            app.display_focus(),
            &DisplayFocus::Command(Box::new(DisplayFocus::TableSelector))
        );
        assert_eq!(app.command_name(), Some("test"));
        assert_eq!(
            DisplayFocus::last_focus(app.display_focus()),
            DisplayFocus::TableSelector
        );
        assert!(app.execute_command().is_ok());
    }
}
