use eyre::Result;

use crate::model::app::{state::DisplayFocus, App};

pub(crate) fn handle_delete(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableSelector => app.remove_table(),
        DisplayFocus::TableView => {
            let tv = app.get_selected_table_view_mut()?;
            let (row, col) = tv
                .get_selector_index()
                .ok_or_else(|| eyre::eyre!("No cell selected"))?;

            tv.update_cell(row, col, "")?;
            Ok(())
        }
        _ => Ok(()),
    }
}
