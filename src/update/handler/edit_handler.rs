use eyre::Result;

use crate::model::app::{
    state::{AppCommand, DisplayFocus},
    App,
};

pub(crate) fn edit_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableList => {
            app.clear_user_input();
            app.focus_command(gen_edit_table_name_command());
            Ok(())
        }
        DisplayFocus::TableView => {
            app.clear_user_input();
            app.focus_command(gen_edit_cell_command());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn gen_edit_table_name_command() -> AppCommand {
    AppCommand::new(
        "Edit Table Name",
        Box::new(|app| {
            app.update_table_name(&app.get_user_input().to_string())?;
            app.clear_user_input();
            Ok(())
        }),
    )
}

fn gen_edit_cell_command() -> AppCommand {
    AppCommand::new(
        "Edit Cell",
        Box::new(|app| {
            app.update_cell_value(&app.get_user_input().to_string())?;
            app.clear_user_input();
            Ok(())
        }),
    )
}
