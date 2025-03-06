use eyre::Result;

use crate::model::app::{state::DisplayFocus, App};

pub(crate) fn handle_cancel(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            app.clear_command();
            app.focus_last()
        }
        DisplayFocus::Error(_) => {
            app.clear_error_message();
            app.focus_last()
        }
        DisplayFocus::TableSelector | DisplayFocus::TableView | DisplayFocus::Exit(_) => {
            app.focus_last()
        }
    }
}
