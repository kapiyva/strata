use eyre::Result;

use crate::model::app::{
    state::{AppCommand, DisplayFocus},
    App,
};

pub(crate) fn enter_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            app.execute_command()?;
            app.clear_user_input();
            Ok(())
        }
        DisplayFocus::TableList => app.select_table(),
        DisplayFocus::TableView => {
            app.clear_user_input();
            app.focus_command(AppCommand::new(
                "Edit Cell",
                Box::new(|app| {
                    let input = app.get_user_input().to_string();
                    app.update_cell_value(&input)
                }),
            ));
            Ok(())
        }
        DisplayFocus::Error(_) => {
            app.clear_error_message();
            app.focus_last()
        }
        _ => Ok(()),
    }
}
