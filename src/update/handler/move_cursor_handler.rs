use std::mem;

use eyre::Result;

use crate::{
    message::MoveDirection,
    model::app::{state::DisplayFocus, App},
};

pub(crate) fn handle_move_cursor(app: &mut App, direction: MoveDirection) -> Result<()> {
    match app.get_display_focus() {
        DisplayFocus::TableSelector => match direction {
            MoveDirection::Up => {
                let sl = app.get_table_selector_mut();
                *sl = mem::take(sl).select_prev();
                Ok(())
            }
            MoveDirection::Down => {
                let sl = app.get_table_selector_mut();
                *sl = mem::take(sl).select_prev();
                Ok(())
            }
            MoveDirection::Right => app.focus_table_view(),
            _ => Ok(()),
        },
        DisplayFocus::TableView => {
            let tv = app.get_selected_table_view_mut()?;
            let owned_tv = mem::take(tv);

            *tv = match direction {
                MoveDirection::Up => owned_tv.move_selector(-1, 0)?,
                MoveDirection::Down => owned_tv.move_selector(1, 0)?,
                MoveDirection::Left => owned_tv.move_selector(0, -1)?,
                MoveDirection::Right => owned_tv.move_selector(0, 1)?,
            };
            Ok(())
        }
        _ => Ok(()),
    }
}
