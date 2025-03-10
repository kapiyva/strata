use eyre::{OptionExt, Result};

use crate::{
    app::{
        component::{command::CommandPopup, table_selector::TableName},
        App,
    },
    error::StrataError,
};

pub(crate) fn handle_edit_table_name(app: &mut App) -> Result<&mut App> {
    app.focus_command(CommandPopup::new(
        "Edit Table Name",
        "",
        Box::new(|input, app| {
            let table_name = TableName::from(input.to_string())?;
            let selected_index = app
                .table_selector()
                .selected_index()
                .ok_or_eyre(StrataError::NoTableSelected)?;

            app.table_selector_mut()
                .update_table(selected_index, table_name)?;
            app.focus_table_selector();
            Ok(())
        }),
    ));
    Ok(app)
}

#[cfg(test)]
mod tests {
    use crate::test_util::{input_to_command, setup_sample_app};

    use super::*;

    #[test]
    fn test_command() {
        let mut app = setup_sample_app();

        handle_edit_table_name(&mut app).unwrap();
        input_to_command(&mut app, "new_table_name");
        app.execute_command().unwrap();

        assert_eq!(
            app.table_selector()
                .selected_table_name()
                .map(TableName::as_str),
            Some("new_table_name")
        );
    }
}
