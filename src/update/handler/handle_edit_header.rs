use eyre::Result;

use crate::app::{component::command::CommandPopup, App};

pub(crate) fn handle_edit_header(app: &mut App) -> Result<&mut App> {
    let tv = app.selected_table_view()?;
    let (_, col) = tv
        .selected_index()
        .ok_or_else(|| eyre::eyre!("No column selected"))?;
    let initial_value = app
        .selected_table_view()?
        .headers()
        .get(col)
        .unwrap_or(&String::from(""))
        .clone();

    app.focus_command(CommandPopup::new(
        "Edit Header",
        &initial_value,
        Box::new(|input, app| {
            let tv = app.selected_table_view_mut()?;
            let (_, col) = tv
                .selected_index()
                .ok_or_else(|| eyre::eyre!("No column selected"))?;

            tv.update_header(col, input)?;
            app.focus_last()?;
            Ok(())
        }),
    ));
    Ok(app)
}
