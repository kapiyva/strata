use eyre::Result;

use crate::app::{component::command::CommandPopup, App};

pub(crate) fn handle_edit_cell(app: &mut App) -> Result<()> {
    app.focus_command(CommandPopup::new(
        "Edit Cell",
        "",
        Box::new(|input, app| {
            let tv = app.selected_table_view_mut()?;
            let (row, col) = tv
                .selected_index()
                .ok_or_else(|| eyre::eyre!("No cell selected"))?;

            tv.update_cell(row, col, input)?;
            app.focus_table_view()?;
            Ok(())
        }),
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_util::{input_to_command, setup_sample_app};

    use super::*;

    #[test]
    fn test_edit_cell_command() {
        let mut app = setup_sample_app();
        app.focus_table_view().unwrap();

        handle_edit_cell(&mut app).unwrap();
        input_to_command(&mut app, "new cell value");
        app.execute_command().unwrap();

        assert_eq!(
            app.selected_table_view().unwrap().cell_value(0, 0).unwrap(),
            "new cell value"
        );
    }
}
