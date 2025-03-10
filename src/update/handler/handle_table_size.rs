use eyre::Result;

use crate::{app::App, error::StrataError};

pub fn handle_expand_row(app: &mut App) -> Result<&mut App> {
    app.selected_table_view_mut()?.expand_row();
    Ok(app)
}

pub fn handle_collapse_row(app: &mut App) -> Result<&mut App> {
    let tv = app.selected_table_view_mut()?;
    let (row, _) = tv.selected_index().ok_or(StrataError::NoCellSelected)?;

    tv.collapse_row(row)?;
    Ok(app)
}

pub fn handle_expand_col(app: &mut App) -> Result<&mut App> {
    app.selected_table_view_mut()?.expand_col();
    Ok(app)
}

pub fn handle_collapse_col(app: &mut App) -> Result<&mut App> {
    let tv = app.selected_table_view_mut()?;
    let (_, col) = tv.selected_index().ok_or(StrataError::NoCellSelected)?;

    tv.collapse_col(col)?;
    Ok(app)
}
