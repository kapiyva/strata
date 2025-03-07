use eyre::{bail, OptionExt, Result};

use crate::{
    app::{
        component::{command::CommandPopup, table_selector::TableName},
        display_focus::DisplayFocus,
        App,
    },
    error::StrataError,
};

pub(crate) fn handle_jump(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector => {
            app.focus_command(select_table_command());
            Ok(())
        }
        DisplayFocus::TableView => {
            app.focus_command(select_cell_command());
            Ok(())
        }
        _ => bail!(StrataError::InvalidOperationCall {
            operation: "Jump".to_string(),
            focus: app.display_focus().to_string()
        }),
    }
}

fn select_table_command() -> CommandPopup {
    CommandPopup::new(
        "Jump [input table name e.g. table1]",
        "",
        Box::new(|input, app| {
            let table_name = TableName::from(input)?;
            app.table_selector_mut().select_by_name(&table_name)?;
            app.focus_table_view()?;
            Ok(())
        }),
    )
}

fn select_cell_command() -> CommandPopup {
    CommandPopup::new(
        "Jump [input row and col index e.g. 1 2]",
        "",
        Box::new(|input, app| {
            let index_str = input.to_string();
            let (row, col) = index_str
                .split_once(" ")
                .map(|(row, col)| (row.parse::<usize>(), col.parse::<usize>()))
                .ok_or_eyre(StrataError::StringParseError(index_str))?;

            app.selected_table_view_mut()?.select_cell(row?, col?)?;
            app.focus_table_view()?;
            Ok(())
        }),
    )
}
