use eyre::{bail, Result};

use crate::{
    error::StrataError,
    model::app::{state::DisplayFocus, App, AppCommand},
};

pub(crate) fn jump_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableView => {
            app.clear_user_input();
            app.focus_command(gen_command());
            Ok(())
        }
        _ => bail!(StrataError::InvalidOperationCall {
            operation: "Jump".to_string(),
            focus: app.get_display_focus().to_string()
        }),
    }
}

fn gen_command() -> AppCommand {
    AppCommand::new(
        "Jump",
        Box::new(|app| {
            let input = app.get_user_input().to_string();
            app.jump_cell_selector(&input)?;
            app.clear_user_input();
            app.focus_table_view()?;
            Ok(())
        }),
    )
}
