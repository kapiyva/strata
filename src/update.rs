mod handler;

use crate::{error::StrataError, model::app::App};
use eyre::{bail, Result};
use std::path::PathBuf;

use handler::*;

pub enum Message {
    AddTableMode,
    SelectTableMode,
    SelectCellMode,
    EditHeaderMode,
    EditCellMode,
    NewTable(String),
    OpenCsv(PathBuf),
    Move(MoveDirection),
    SelectTable,
    RemoveTable,
    SaveTable(String),
    ExpandRow,
    CollapseRow,
    ExpandColumn,
    CollapseColumn,
    SaveHeader(String),
    SaveCellValue(String),
    Exiting,
    CancelExit,
    NoOp,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn update(app: &mut App, message: Message) -> Result<()> {
    match message {
        Message::AddTableMode => app.set_add_table_mode(),
        Message::SelectTableMode => app.set_select_table_mode(),
        Message::EditCellMode => app.set_edit_cell_mode(),
        Message::SelectCellMode => app.set_select_cell_mode(),
        Message::NewTable(table_name) => app.add_table(&table_name.to_string()),
        // Message::OpenCsv(path) => model.open_csv(path),
        Message::Move(direction) => move_cursor_handler(app, direction),
        Message::SelectTable => app.select_table(),
        Message::RemoveTable => app.remove_table(),
        // Message::SaveTable(table_name) => model.save_table(&table_name),
        Message::ExpandRow => app.expand_row(),
        Message::CollapseRow => {
            let Some((row, _)) = app.get_selected_cell() else {
                bail!(StrataError::NoCellSelected);
            };
            app.collapse_row(row)
        }
        Message::ExpandColumn => app.expand_col(),
        Message::CollapseColumn => {
            let Some((_, col)) = app.get_selected_cell() else {
                bail!(StrataError::NoCellSelected);
            };
            app.collapse_col(col)
        }
        Message::SaveHeader(header) => app.update_header(&header),
        Message::SaveCellValue(value) => app.update_cell_value(&value),
        Message::Exiting => app.set_exit(true),
        Message::CancelExit => app.set_exit(false),
        Message::NoOp => Ok(()),
        _ => bail!("Message handler not implemented"),
    }
}
