use eyre::{OptionExt, Result};

use crate::{
    error::StrataError,
    model::{
        app::{state::DisplayFocus, App},
        component::{command::AppCommand, table_selector::TableName},
    },
};

pub(crate) fn handle_edit(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableSelector => {
            app.focus_command(edit_table_name_command());
            Ok(())
        }
        DisplayFocus::TableView => {
            app.focus_command(edit_cell_command());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn edit_table_name_command() -> AppCommand {
    AppCommand::new(
        "Edit Table Name",
        "",
        Box::new(|input, app| {
            let table_name = TableName::from(input.to_string())?;
            let selected_index = app
                .get_table_selector()
                .get_selected_index()
                .ok_or_eyre(StrataError::NoTableSelected)?;

            app.get_table_selector_mut()
                .update_table(selected_index, table_name)?;
            Ok(())
        }),
    )
}

fn edit_cell_command() -> AppCommand {
    AppCommand::new(
        "Edit Cell",
        "",
        Box::new(|input, app| {
            let tv = app.get_selected_table_view_mut()?;
            let (row, col) = tv
                .get_selector_index()
                .ok_or_else(|| eyre::eyre!("No cell selected"))?;

            tv.update_cell(row, col, input)?;
            Ok(())
        }),
    )
}
