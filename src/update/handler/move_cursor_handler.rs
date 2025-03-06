use eyre::Result;

use crate::{
    message::MoveDirection,
    model::app::{state::DisplayFocus, App},
};

pub(crate) fn handle_move_cursor(app: &mut App, direction: MoveDirection) -> Result<()> {
    match app.display_focus() {
        DisplayFocus::TableSelector => match direction {
            MoveDirection::Up => {
                app.table_selector_mut().select_prev();
                Ok(())
            }
            MoveDirection::Down => {
                app.table_selector_mut().select_next();
                Ok(())
            }
            MoveDirection::Right => {
                app.focus_table_view()?;
                Ok(())
            }
            _ => Ok(()),
        },
        DisplayFocus::TableView => {
            let tv = app.selected_table_view_mut()?;

            match direction {
                MoveDirection::Up => tv.move_selector(-1, 0)?,
                MoveDirection::Down => tv.move_selector(1, 0)?,
                MoveDirection::Left => tv.move_selector(0, -1)?,
                MoveDirection::Right => tv.move_selector(0, 1)?,
            };
            Ok(())
        }
        _ => Ok(()),
    }
}
