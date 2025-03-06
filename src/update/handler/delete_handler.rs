use eyre::Result;

use crate::app::{display_focus::DisplayFocus, App};

pub(crate) fn handle_delete(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector => app.remove_table(),
        DisplayFocus::TableView => {
            let tv = app.selected_table_view_mut()?;
            let (row, col) = tv
                .selected_index()
                .ok_or_else(|| eyre::eyre!("No cell selected"))?;

            tv.update_cell(row, col, "")?;
            Ok(())
        }
        _ => Ok(()),
    }
}
