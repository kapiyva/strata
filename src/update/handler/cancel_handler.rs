use eyre::Result;

use crate::model::app::{state::DisplayFocus, App};

pub(crate) fn cancel_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            app.clear_command();
            app.clear_user_input();
            app.focus_last()
        }
        DisplayFocus::Error(_) => {
            app.clear_error_message();
            app.focus_last()
        }
        DisplayFocus::TableList | DisplayFocus::TableView | DisplayFocus::Exit(_) => {
            app.focus_last()
        }
    }
}
