use eyre::{OptionExt, Result};

use crate::{
    app::{component::command::CommandPopup, App},
    error::StrataError,
};

pub(crate) fn handle_jump_cell(app: &mut App) -> Result<&mut App> {
    app.focus_command(CommandPopup::new(
        "Jump [input row and col index e.g. 1 2]",
        "",
        Box::new(|input, app| {
            let index_str = input.to_string();
            let (row, col) = index_str
                .split_once(" ")
                .map(|(row, col)| (row.parse::<usize>(), col.parse::<usize>()))
                .ok_or_eyre(StrataError::StringParseError(index_str))?;

            app.selected_table_view_mut()?.select_cell(row?, col?)?;
            app.focus_table_view()?;
            Ok(())
        }),
    ));
    Ok(app)
}

#[cfg(test)]
mod tests {
    use crate::{
        app::display_focus::DisplayFocus,
        test_util::{input_to_command, setup_sample_app},
    };

    use super::*;

    #[test]
    fn test_handle_jump_cell() {
        let mut app = setup_sample_app();
        handle_jump_cell(&mut app).unwrap();
        input_to_command(&mut app, "10 10");

        app.execute_command().unwrap();

        assert_eq!(
            app.selected_table_view().unwrap().selected_index(),
            Some((10, 10))
        );
        assert_eq!(*app.display_focus(), DisplayFocus::TableView);
    }

    #[test]
    fn test_handle_jump_cell_parse_error() {
        // input is less than 2
        let mut app = setup_sample_app();
        handle_jump_cell(&mut app).unwrap();
        input_to_command(&mut app, "1");

        let err = app.execute_command();
        assert!(err.is_err());

        // input is more than 2
        let mut app = setup_sample_app();
        handle_jump_cell(&mut app).unwrap();
        input_to_command(&mut app, "1 1 1");

        let err = app.execute_command();
        assert!(err.is_err());

        // input is not a number
        let mut app = setup_sample_app();
        handle_jump_cell(&mut app).unwrap();
        input_to_command(&mut app, "1 a");

        let err = app.execute_command();
        assert!(err.is_err());
    }

    #[test]
    fn test_handle_jump_cell_invalid_index() {
        // invalid row index
        let mut app = setup_sample_app();
        handle_jump_cell(&mut app).unwrap();
        input_to_command(&mut app, "11 0");

        let Err(err) = app.execute_command() else {
            panic!("Expected an error");
        };

        assert_eq!(
            err.to_string(),
            StrataError::InvalidRowIndex {
                max: 10,
                requested: 11
            }
            .to_string()
        );

        // invalid column index
        let mut app = setup_sample_app();
        handle_jump_cell(&mut app).unwrap();
        input_to_command(&mut app, "0 11");

        let Err(err) = app.execute_command() else {
            panic!("Expected an error");
        };

        assert_eq!(
            err.to_string(),
            StrataError::InvalidColumnIndex {
                max: 10,
                requested: 11
            }
            .to_string()
        );
    }
}
