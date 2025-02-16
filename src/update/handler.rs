use eyre::Result;

use crate::model::app::{state::DisplayState, App};

use super::MoveDirection;

pub(super) fn move_cursor_handler(model: &mut App, direction: MoveDirection) -> Result<()> {
    match model.get_display_state() {
        DisplayState::SelectTable(_) => match direction {
            MoveDirection::Up => model.up_table_selector(),
            MoveDirection::Down => model.down_table_selector(),
            _ => Ok(()),
        },
        DisplayState::SelectCell(_) => match direction {
            MoveDirection::Up => model.move_cell_selector(0, -1),
            MoveDirection::Down => model.move_cell_selector(0, 1),
            MoveDirection::Left => model.move_cell_selector(-1, 0),
            MoveDirection::Right => model.move_cell_selector(1, 0),
        },
        _ => Ok(()),
    }
}
