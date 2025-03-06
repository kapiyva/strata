use eyre::Result;

use crate::model::{
    app::{state::DisplayFocus, App},
    component::command::AppCommand,
};

pub(crate) fn handle_hyper_edit(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableView => {
            app.focus_command(gen_command());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn gen_command() -> AppCommand {
    AppCommand::new(
        "Edit Table Header",
        "",
        Box::new(|input, app| {
            let tv = app.get_selected_table_view_mut()?;
            let (_, col) = tv
                .get_selector_index()
                .ok_or_else(|| eyre::eyre!("No column selected"))?;

            tv.update_header(col, input)?;
            Ok(())
        }),
    )
}
