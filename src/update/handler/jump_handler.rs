use eyre::{bail, Result};

use crate::{error::StrataError, model::app::App};

pub(crate) fn jump_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        // DisplayFocus::TableView => {
        //     app.clear_user_input();
        //     app.focus_command(gen_edit_cell_command());
        //     Ok(())
        // }
        _ => bail!(StrataError::InvalidOperationCall {
            operation: "Jump".to_string(),
            mode: app.get_display_focus().to_string()
        }),
    }
}
