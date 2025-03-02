use eyre::Result;

use crate::model::app::{
    state::{AppCommand, DisplayFocus},
    App,
};

pub(crate) fn hyper_edit_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableView => {
            app.clear_user_input();
            app.focus_command(gen_command());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn gen_command() -> AppCommand {
    AppCommand::new(
        "Edit Table Header",
        Box::new(|app| {
            app.update_header(&app.get_user_input().to_string())?;
            app.clear_user_input();
            Ok(())
        }),
    )
}
