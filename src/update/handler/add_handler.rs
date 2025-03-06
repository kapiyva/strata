use eyre::Result;

use crate::model::{
    app::{state::DisplayFocus, App},
    component::command::AppCommand,
};

pub(crate) fn handle_add(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableSelector => {
            app.focus_command(AppCommand::new(
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
    use std::mem;

    use crate::model::component::table_selector::TableName;

    use super::*;

    fn input(app: &mut App, input: &str) {
        let command = app.get_command_mut().unwrap();
        for c in input.chars() {
            *command = mem::take(command).input(c);
        }
    }

    #[test]
    fn test_add_handler() {
        let mut app = App::new();
        // focus command
        handle_add(&mut app).unwrap();
        // input table name
        let table_name = "table1";
        input(&mut app, table_name);
        // execute command
        app.execute_command().unwrap();

        assert_eq!(*app.get_display_focus(), DisplayFocus::TableView);
        assert_eq!(app.get_table_selector().get_table_list().len(), 1);
        assert_eq!(
            app.get_table_selector().get_selected_table_name(),
            Some(&TableName::from(table_name).unwrap())
        );
    }
}
