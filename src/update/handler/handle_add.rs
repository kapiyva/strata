use eyre::Result;

use crate::app::{component::command::CommandPopup, display_focus::DisplayFocus, App};

pub(crate) fn handle_add(app: &mut App) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector => {
            app.focus_command(CommandPopup::new(
                "Add Table",
                "",
                Box::new(|input, app| {
                    app.add_table(input)?;
                    app.focus_table_view_by_name(input)?;
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
    use crate::{
        app::component::table_selector::TableName,
        test_util::{input_to_command, setup_sample_app},
    };

    use super::*;

    #[test]
    fn test_add_handler() {
        let mut app = setup_sample_app();
        // focus command
        handle_add(&mut app).unwrap();
        // input table name
        let table_name = "table1";
        input_to_command(&mut app, table_name);
        // execute command
        app.execute_command().unwrap();

        assert_eq!(*app.display_focus(), DisplayFocus::TableView);
        assert_eq!(app.table_selector().table_list().len(), 1);
        assert_eq!(
            app.table_selector().selected_table_name(),
            Some(&TableName::from(table_name).unwrap())
        );
    }
}
