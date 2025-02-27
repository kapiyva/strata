use eyre::Result;

use crate::model::app::{state::DisplayFocus, App};

pub(crate) fn delete_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableList => app.remove_table(),
        DisplayFocus::TableView => app.update_cell_value(""),
        _ => Ok(()),
    }
}
