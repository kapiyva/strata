use eyre::Result;

use crate::app::{
    component::{command::CommandPopup, table_selector::TableName},
    App,
};

pub(crate) fn handle_jump_table(app: &mut App) -> Result<()> {
    app.focus_command(CommandPopup::new(
        "Jump [input table name e.g. table1]",
        "",
        Box::new(|input, app| {
            let table_name = TableName::from(input)?;
            app.table_selector_mut().select_by_name(&table_name)?;
            app.focus_table_view()?;
            Ok(())
        }),
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        app::display_focus::DisplayFocus,
        test_util::{input_to_command, setup_sample_app},
    };

    use super::*;

    #[test]
    fn test_handle_jump_table() {
        let mut app = setup_sample_app();

        handle_jump_table(&mut app).unwrap();
        input_to_command(&mut app, "table2");

        assert_eq!(
            *app.table_selector().selected_table_name().unwrap(),
            TableName::from("table2").unwrap()
        );
        assert_eq!(*app.display_focus(), DisplayFocus::TableView);
    }
}
