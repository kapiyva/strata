use eyre::Result;

use crate::model::app::{state::DisplayFocus, App};

pub(crate) fn handle_cancel(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            app.clear_command();
            app.focus_last()?;
            Ok(())
        }
        DisplayFocus::Error(_) => {
            app.get_error_popup_mut().clear();
            app.focus_last()?;
            Ok(())
        }
        DisplayFocus::TableSelector | DisplayFocus::TableView | DisplayFocus::Exit(_) => {
            app.focus_last()?;
            Ok(())
        }
    }
}
