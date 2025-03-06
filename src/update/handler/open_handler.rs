use std::{ffi::OsStr, path::Path};

use eyre::{OptionExt, Result};

use crate::{
    app::{component::command::CommandPopup, display_focus::DisplayFocus, App},
    error::StrataError,
};

pub(crate) fn handle_open(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector => {
            app.focus_command(CommandPopup::new(
                "Open File",
                "",
                Box::new(|input, app| {
                    let path = Path::new(input);
                    let table_name = path
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .ok_or_eyre(StrataError::InvalidTableName)?;
                    app.open_table(&path, true)?;
                    app.focus_table_view_by_name(&table_name)?;
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
    // use super::*;

    // todo: Implement tests
    #[test]
    fn test_handle_open() {}
}
