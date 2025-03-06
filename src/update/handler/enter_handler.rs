use eyre::{OptionExt, Result};

use crate::{
    app::{component::command::CommandPopup, display_focus::DisplayFocus, App},
    error::StrataError,
};

pub(crate) fn handle_enter(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::Command(_) => {
            app.execute_command()?;
            Ok(())
        }
        DisplayFocus::TableSelector => {
            app.focus_table_view()?;
            Ok(())
        }
        DisplayFocus::TableView => {
            app.focus_command(CommandPopup::new(
                "Edit Cell",
                "",
                Box::new(|input, app| {
                    let tv = app.selected_table_view_mut()?;
                    let (row, col) = tv
                        .selected_index()
                        .ok_or_eyre(StrataError::NoCellSelected)?;
                    tv.update_cell(row, col, input)?;
                    Ok(())
                }),
            ));
            Ok(())
        }
        DisplayFocus::Error(_) => {
            app.error_popup_mut().clear();
            app.focus_last()?;
            Ok(())
        }
        _ => Ok(()),
    }
}
