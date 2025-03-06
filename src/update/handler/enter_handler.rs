use eyre::{OptionExt, Result};

use crate::{
    error::StrataError,
    model::{
        app::{state::DisplayFocus, App},
        component::command::AppCommand,
    },
};

pub(crate) fn handle_enter(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            app.execute_command()?;
            Ok(())
        }
        DisplayFocus::TableSelector => {
            app.focus_table_view()?;
            Ok(())
        }
        DisplayFocus::TableView => {
            app.focus_command(AppCommand::new(
                "Edit Cell",
                "",
                Box::new(|input, app| {
                    let tv = app.get_selected_table_view_mut()?;
                    let (row, col) = tv
                        .get_selector_index()
                        .ok_or_eyre(StrataError::NoCellSelected)?;
                    tv.update_cell(row, col, input)?;
                    Ok(())
                }),
            ));
            Ok(())
        }
        DisplayFocus::Error(_) => {
            app.get_error_popup_mut().clear();
            app.focus_last()?;
            Ok(())
        }
        _ => Ok(()),
    }
}
