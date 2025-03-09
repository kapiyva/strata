use std::path::Path;

use eyre::Result;

use crate::app::{component::command::CommandPopup, App};

pub(crate) fn handle_save(app: &mut App) -> Result<()> {
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
