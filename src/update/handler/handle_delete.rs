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

#[cfg(test)]
mod tests {
    use crate::test_util::setup_sample_app;

    use super::*;

    #[test]
    fn test_delete_cell_command() {
        let mut app = setup_sample_app();
        app.focus_table_view().unwrap();

        handle_delete(&mut app).unwrap();

        assert_eq!(
            app.selected_table_view().unwrap().cell_value(0, 0).unwrap(),
            ""
        );
    }

    #[test]
    fn test_delete_table_command() {
        let mut app = setup_sample_app();

        handle_delete(&mut app).unwrap();

        assert_eq!(app.table_selector().table_list().len(), 1);
    }
}
