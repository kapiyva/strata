use eyre::Result;

use crate::{
    error::StrataError,
    model::{app::App, component::command::AppCommand},
};

pub fn handle_expand_row(app: &mut App) -> Result<()> {
    app.selected_table_view_mut()?.expand_row()?;
    Ok(())
}

pub fn handle_collapse_row(app: &mut App) -> Result<()> {
    let tv = app.selected_table_view_mut()?;
    let (row, _) = tv.selected_index().ok_or(StrataError::NoCellSelected)?;

    tv.collapse_row(row)?;
    Ok(())
}

pub fn handle_expand_col(app: &mut App) -> Result<()> {
    app.focus_command(AppCommand::new(
        "Header Name",
        "",
        Box::new(|input, app| {
            app.selected_table_view_mut()?.expand_col(&input)?;
            app.focus_last()?;
            Ok(())
        }),
    ));
    Ok(())
}

pub fn handle_collapse_col(app: &mut App) -> Result<()> {
    let tv = app.selected_table_view_mut()?;
    let (_, col) = tv.selected_index().ok_or(StrataError::NoCellSelected)?;

    tv.collapse_col(col)?;
    Ok(())
}
