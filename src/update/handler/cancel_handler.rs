use eyre::Result;

use crate::app::{display_focus::DisplayFocus, App};

pub(crate) fn handle_cancel(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::Command(_) => {
            app.clear_command();
            app.focus_last()?;
            Ok(())
        }
        DisplayFocus::Error(_) => {
            app.error_popup_mut().clear();
            app.focus_last()?;
            Ok(())
        }
        DisplayFocus::TableSelector | DisplayFocus::TableView | DisplayFocus::Exit(_) => {
            app.focus_last()?;
            Ok(())
        }
    }
}
