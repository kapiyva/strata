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
                    app.focus_table_view()?;
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

#[cfg(test)]
mod tests {
    use crate::test_util::{input_to_command, setup_sample_app};

    use super::*;

    #[test]
    fn test_edit_cell_command() {
        let mut app = setup_sample_app();
        app.focus_table_view().unwrap();

        handle_enter(&mut app).unwrap();
        input_to_command(&mut app, "new cell value");
        handle_enter(&mut app).unwrap();

        assert_eq!(
            app.selected_table_view().unwrap().cell_value(0, 0).unwrap(),
            "new cell value"
        );
    }
}
