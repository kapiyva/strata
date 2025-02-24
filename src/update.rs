use crate::{error::StrataError, message::Message, model::app::App};
use eyre::{bail, Result};

use handler::*;

pub fn update(app: &mut App, message: Message) -> Result<()> {
    match message {
        Message::AddTableMode => {
            app.clear_user_input();
            app.set_add_table_mode()
        }
        Message::SelectTableMode => app.set_select_table_mode(),
        Message::EditCellMode => {
            app.clear_user_input();
            app.set_edit_cell_mode()
        }
        Message::EditHeaderMode => {
            app.clear_user_input();
            app.set_edit_header_mode()
        }
        Message::SelectCellMode => app.set_select_cell_mode(),
        Message::Input(c) => {
            app.push_user_input(c);
            Ok(())
        }
        Message::PopInput => {
            app.pop_user_input();
            Ok(())
        }
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

mod handler {
    use eyre::Result;

    use crate::{
        message::MoveDirection,
        model::app::{state::DisplayMode, App},
    };

    pub(super) fn move_cursor_handler(model: &mut App, direction: MoveDirection) -> Result<()> {
        match model.get_display_mode() {
            DisplayMode::SelectTable => match direction {
                MoveDirection::Up => model.up_table_selector(),
                MoveDirection::Down => model.down_table_selector(),
                _ => Ok(()),
            },
            DisplayMode::SelectCell => match direction {
                MoveDirection::Up => model.move_cell_selector(-1, 0),
                MoveDirection::Down => model.move_cell_selector(1, 0),
                MoveDirection::Left => model.move_cell_selector(0, -1),
                MoveDirection::Right => model.move_cell_selector(0, 1),
            },
            _ => Ok(()),
        }
    }
}
