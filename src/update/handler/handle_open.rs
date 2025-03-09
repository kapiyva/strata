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
    use super::*;

    use crate::{
        app::component::table_selector::TableName,
        test_util::{input_to_command, setup_sample_app},
    };

    #[test]
    fn test_handle_open() {
        let mut app = setup_sample_app();

        handle_open(&mut app).unwrap();
        input_to_command(&mut app, "tests/data/fluits.csv");
        app.execute_command().unwrap();

        assert_eq!(*app.display_focus(), DisplayFocus::TableView);
        assert_eq!(
            *app.table_selector().selected_table_name().unwrap(),
            TableName::from("fluits").unwrap()
        );
        let tv = app.selected_table_view().unwrap();
        assert_eq!(*tv.header(), vec!["fluits", "price"]);
        assert_eq!(tv.cell_value(0, 0).unwrap(), "apple");
        assert_eq!(tv.cell_value(0, 1).unwrap(), "100");
        assert_eq!(tv.cell_value(1, 0).unwrap(), "orange");
        assert_eq!(tv.cell_value(2, 0).unwrap(), "grape");
    }
}
