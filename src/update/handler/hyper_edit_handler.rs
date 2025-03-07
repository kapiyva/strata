use eyre::Result;

use crate::app::{component::command::CommandPopup, display_focus::DisplayFocus, App};

pub(crate) fn handle_hyper_edit(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableView => {
            app.focus_command(gen_command());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn gen_command() -> CommandPopup {
    CommandPopup::new(
        "Edit Table Header",
        "",
        Box::new(|input, app| {
            let tv = app.selected_table_view_mut()?;
            let (_, col) = tv
                .selected_index()
                .ok_or_else(|| eyre::eyre!("No column selected"))?;

            tv.update_header(col, input)?;
            app.focus_last()?;
            Ok(())
        }),
    )
}
