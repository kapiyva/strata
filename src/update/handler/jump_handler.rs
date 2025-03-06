use eyre::{bail, OptionExt, Result};

use crate::{
    app::{component::command::CommandPopup, display_focus::DisplayFocus, App},
    error::StrataError,
};

pub(crate) fn handle_jump(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableView => {
            app.focus_command(gen_command());
            Ok(())
        }
        _ => bail!(StrataError::InvalidOperationCall {
            operation: "Jump".to_string(),
            focus: app.display_focus().to_string()
        }),
    }
}

fn gen_command() -> CommandPopup {
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
