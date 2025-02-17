use eyre::Result;

use crate::model::app::{state::DisplayMode, App};

use super::MoveDirection;

pub(super) fn move_cursor_handler(model: &mut App, direction: MoveDirection) -> Result<()> {
    match model.get_display_mode() {
        DisplayMode::SelectTable => match direction {
            MoveDirection::Up => model.up_table_selector(),
            MoveDirection::Down => model.down_table_selector(),
            _ => Ok(()),
        },
        DisplayMode::SelectCell => match direction {
            MoveDirection::Up => model.move_cell_selector(0, -1),
            MoveDirection::Down => model.move_cell_selector(0, 1),
            MoveDirection::Left => model.move_cell_selector(-1, 0),
            MoveDirection::Right => model.move_cell_selector(1, 0),
        },
        _ => Ok(()),
    }
}
