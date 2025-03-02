use std::{ffi::OsStr, path::Path};

use eyre::{OptionExt, Result};

use crate::{
    error::StrataError,
    model::app::{
        state::{AppCommand, DisplayFocus},
        App,
    },
};

pub(crate) fn open_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableList => {
            app.clear_user_input();
            app.focus_command(AppCommand::new(
                "Open File",
                Box::new(|app| {
                    let input = app.get_user_input().to_string();
                    let path = Path::new(&input);
                    let table_name = path
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .ok_or_eyre(StrataError::InvalidTableName)?;
                    app.open_table(&path)?;
                    app.focus_table_view_by_name(&table_name)?;
                    app.clear_user_input();
                    Ok(())
                }),
            ));
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    // use crate::model::table::TableName;

    // use super::*;

    #[test]
    fn test_open_handler() {}
}
