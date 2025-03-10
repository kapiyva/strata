use eyre::Result;

use crate::app::{component::command::CommandPopup, App};

pub(crate) fn handle_add_table(app: &mut App) -> Result<&mut App> {
    app.focus_command(CommandPopup::new(
        "Add Table",
        "",
        Box::new(|input, app| {
            app.add_table(input)?;
            app.focus_table_view_by_name(input)?;
            Ok(())
        }),
    ));
    Ok(app)
}

#[cfg(test)]
mod tests {
    use crate::{
        app::{component::table_selector::TableName, display_focus::DisplayFocus},
        test_util::{input_to_command, setup_sample_app},
    };

    use super::*;

    #[test]
    fn test_add_handler() {
        let mut app = setup_sample_app();
        // focus command
        handle_add_table(&mut app).unwrap();
        // input table name
        let table_name = "table3";
        input_to_command(&mut app, table_name);
        // execute command
        app.execute_command().unwrap();

        assert_eq!(*app.display_focus(), DisplayFocus::TableView);
        assert_eq!(app.table_selector().table_list().len(), 3);
        assert_eq!(
            app.table_selector().selected_table_name(),
            Some(&TableName::from(table_name).unwrap())
        );
    }
}
