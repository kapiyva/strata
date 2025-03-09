pub mod app;
pub mod error;
pub mod message;
pub mod update;
pub mod view;

#[cfg(test)]
pub mod test_util {
    use crate::app::{component::table_selector::TableName, App};

    /// Setup a sample app with two tables
    ///
    /// DisplayFocus: TableSelector
    /// TableSelector: table1(selected), table2
    pub fn setup_sample_app() -> App {
        let mut app = App::new();

        let table_name_1 = "table1";
        app.add_table(table_name_1).unwrap();
        app.selected_table_view_mut()
            .and_then(|tv| Ok(tv.expand_row()))
            .and_then(|tv| Ok(tv.expand_col()))
            .and_then(|tv| tv.update_cell(0, 0, "cell 0-0"))
            .and_then(|tv| tv.update_cell(0, 1, "cell 0-1"))
            .and_then(|tv| tv.update_cell(1, 0, "cell 1-0"))
            .and_then(|tv| tv.update_cell(1, 1, "cell 1-1"))
            .unwrap();

        let table_name_2 = "table2";
        app.add_table(table_name_2).unwrap();
        app.selected_table_view_mut()
            .and_then(|tv| Ok(tv.expand_row()))
            .and_then(|tv| Ok(tv.expand_col()))
            .and_then(|tv| tv.update_cell(0, 0, "cell 0-0"))
            .and_then(|tv| tv.update_cell(0, 1, "cell 0-1"))
            .and_then(|tv| tv.update_cell(1, 0, "cell 1-0"))
            .and_then(|tv| tv.update_cell(1, 1, "cell 1-1"))
            .unwrap();

        app.focus_table_selector();
        app.table_selector_mut()
            .select_by_name(&TableName::from(table_name_1).unwrap())
            .unwrap();

        app
    }

    pub fn input_to_command(app: &mut App, input: &str) {
        let command = app.command_mut().unwrap();
        for c in input.chars() {
            command.input(c);
        }
    }
}
