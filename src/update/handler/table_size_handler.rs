use std::mem;

use eyre::Result;

use crate::{
    error::StrataError,
    model::{app::App, component::command::AppCommand},
};

pub fn handle_expand_row(app: &mut App) -> Result<()> {
    let tv = app.get_selected_table_view_mut()?;

    *tv = mem::take(tv).expand_row()?;
    Ok(())
}

pub fn handle_collapse_row(app: &mut App) -> Result<()> {
    let tv = app.get_selected_table_view_mut()?;
    let (row, _) = tv.get_selector_index().ok_or(StrataError::NoCellSelected)?;

    *tv = mem::take(tv).collapse_row(row)?;
    Ok(())
}

pub fn handle_expand_col(app: &mut App) -> Result<()> {
    app.focus_command(AppCommand::new(
        "Header Name",
        "",
        Box::new(|input, app| {
            let tv = app.get_selected_table_view_mut()?;

            *tv = mem::take(tv).expand_col(&input)?;
            app.focus_last()
        }),
    ));
    Ok(())
}

pub fn handle_collapse_col(app: &mut App) -> Result<()> {
    let tv = app.get_selected_table_view_mut()?;
    let (_, col) = tv.get_selector_index().ok_or(StrataError::NoCellSelected)?;

    *tv = mem::take(tv).collapse_col(col)?;
    Ok(())
}
