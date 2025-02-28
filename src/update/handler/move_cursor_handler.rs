use eyre::Result;

use crate::{
    message::MoveDirection,
    model::app::{state::DisplayFocus, App},
};

pub(crate) fn move_cursor_handler(app: &mut App, direction: MoveDirection) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableList => match direction {
            MoveDirection::Up => app.up_table_selector(),
            MoveDirection::Down => app.down_table_selector(),
            MoveDirection::Right => app.focus_table_view(),
            _ => Ok(()),
        },
        DisplayFocus::TableView => match direction {
            MoveDirection::Up => app.move_cell_selector(-1, 0),
            MoveDirection::Down => app.move_cell_selector(1, 0),
            MoveDirection::Left => app.move_cell_selector(0, -1),
            MoveDirection::Right => app.move_cell_selector(0, 1),
        },
        _ => Ok(()),
    }
}
