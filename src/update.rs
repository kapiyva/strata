mod handler;

use crate::model::{app::App, table::TableName};
use eyre::{bail, Result};
use std::path::PathBuf;

use handler::*;

pub enum Message {
    AddTableMode,
    SelectTableMode,
    EditCellMode,
    NewTable(String),
    OpenCsv(PathBuf),
    Move(MoveDirection),
    SelectTable,
    RemoveTable,
    SaveTable(TableName),
    SaveCellValue(String),
    Exiting,
    CancelExit,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn update(app: &mut App, message: Message) -> Result<()> {
    match message {
        Message::AddTableMode => app.set_state_add_table(),
        Message::SelectTableMode => app.set_state_select_table(),
        Message::EditCellMode => app.set_state_edit_cell(),
        Message::NewTable(table_name) => app.add_table(&table_name),
        // Message::OpenCsv(path) => model.open_csv(path),
        Message::Move(direction) => move_cursor_handler(app, direction),
        Message::SelectTable => app.select_table(),
        Message::RemoveTable => app.remove_table(),
        // Message::SaveTable(table_name) => model.save_table(&table_name),
        Message::SaveCellValue(value) => app.update_cell(&value),
        Message::Exiting => app.set_exit(true),
        Message::CancelExit => app.set_exit(false),
        _ => bail!("Message handler not implemented"),
    }
}
