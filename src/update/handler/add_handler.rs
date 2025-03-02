use eyre::Result;

use crate::model::app::{
    state::{AppCommand, DisplayFocus},
    App,
};

pub(crate) fn add_handler(app: &mut App) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableList => {
            app.clear_user_input();
            app.focus_command(AppCommand::new(
                "Add Table",
                Box::new(|app| {
                    let input = app.get_user_input().to_string();
                    app.add_table(&input)?;
                    app.focus_table_view_by_name(&input)?;
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
    use crate::model::table::TableName;

    use super::*;

    #[test]
    fn test_add_handler() {
        let mut app = App::new();
        // focus add table command
        add_handler(&mut app).unwrap();
        // input table name
        let table_name = "table1";
        table_name.chars().for_each(|c| app.push_user_input(c));
        // execute command
        app.execute_command().unwrap();

        assert_eq!(*app.get_display_focus(), DisplayFocus::TableView);
        assert_eq!(app.get_table_name_list().len(), 1);
        assert_eq!(
            app.get_selected_table_name(),
            Some(&TableName::from(table_name).unwrap())
        );
    }
}
