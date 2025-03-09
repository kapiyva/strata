use eyre::{OptionExt, Result};

use crate::{
    app::{
        component::{command::CommandPopup, table_selector::TableName},
        display_focus::DisplayFocus,
        App,
    },
    error::StrataError,
};

pub(crate) fn handle_edit(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector => {
            app.focus_command(edit_table_name_command());
            Ok(())
        }
        DisplayFocus::TableView => {
            app.focus_command(edit_cell_command());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn edit_table_name_command() -> CommandPopup {
    CommandPopup::new(
        "Edit Table Name",
        "",
        Box::new(|input, app| {
            let table_name = TableName::from(input.to_string())?;
            let selected_index = app
                .table_selector()
                .selected_index()
                .ok_or_eyre(StrataError::NoTableSelected)?;

            app.table_selector_mut()
                .update_table(selected_index, table_name)?;
            app.focus_table_selector();
            Ok(())
        }),
    )
}

fn edit_cell_command() -> CommandPopup {
    CommandPopup::new(
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
    )
}

#[cfg(test)]
mod tests {
    use crate::test_util::{input_to_command, setup_sample_app};

    use super::*;

    #[test]
    fn test_edit_cell_command() {
        let mut app = setup_sample_app();
        app.focus_table_view().unwrap();

        handle_edit(&mut app).unwrap();
        input_to_command(&mut app, "new cell value");
        handle_edit(&mut app).unwrap();

        assert_eq!(
            app.selected_table_view().unwrap().cell_value(0, 0).unwrap(),
            "new cell value"
        );
    }
}
