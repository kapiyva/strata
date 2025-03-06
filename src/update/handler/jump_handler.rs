use std::mem;

use eyre::{bail, OptionExt, Result};

use crate::{
    error::StrataError,
    model::{
        app::{state::DisplayFocus, App},
        component::command::AppCommand,
    },
};

pub(crate) fn handle_jump(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableView => {
            app.focus_command(gen_command());
            Ok(())
        }
        _ => bail!(StrataError::InvalidOperationCall {
            operation: "Jump".to_string(),
            focus: app.get_display_focus().to_string()
        }),
    }
}

fn gen_command() -> AppCommand {
    AppCommand::new(
        "Jump",
        "",
        Box::new(|input, app| {
            let index_str = input.to_string();
            let tv = app.get_selected_table_view_mut()?;
            let (row, col) = index_str
                .split_once(" ")
                .map(|(row, col)| (row.parse::<usize>(), col.parse::<usize>()))
                .ok_or_eyre(StrataError::StringParseError(index_str))?;

            *tv = mem::take(tv).select_cell(row?, col?)?;
            app.focus_table_view()?;
            Ok(())
        }),
    )
}
