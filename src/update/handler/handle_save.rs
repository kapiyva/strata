use std::path::Path;

use eyre::Result;

use crate::app::{component::command::CommandPopup, display_focus::DisplayFocus, App};

pub(crate) fn handle_save(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector | DisplayFocus::TableView => {
            app.focus_command(CommandPopup::new(
                "Save File Path",
                "",
                Box::new(|input, app| {
                    let path = Path::new(input);
                    app.selected_table_view()?.save_csv(path)?;
                    app.focus_last()?;
                    Ok(())
                }),
            ));
            Ok(())
        }
        _ => Ok(()),
    }
}
