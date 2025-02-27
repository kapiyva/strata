mod handler;

use crate::{error::StrataError, message::Message, model::app::App};
use eyre::{bail, Result};
use handler::{
    add_handler::add_handler, cancel_handler::cancel_handler, delete_handler::delete_handler,
    edit_handler::edit_handler, enter_handler::enter_handler,
    hyper_edit_handler::hyper_edit_handler, jump_handler::jump_handler,
    move_cursor_handler::move_cursor_handler,
};

pub fn update(app: &mut App, message: Message) -> Result<()> {
    match message {
        Message::Enter => enter_handler(app),
        Message::Cancel => cancel_handler(app),
        Message::Exiting => Ok(app.focus_exit()),
        Message::Move(direction) => move_cursor_handler(app, direction),
        // TODO
        Message::Jump => jump_handler(app),
        Message::Add => add_handler(app),
        // TODO
        // Message::Open => open_handler(app),
        Message::Edit => edit_handler(app),
        Message::HyperEdit => hyper_edit_handler(app),
        Message::Input(c) => {
            app.push_user_input(c);
            Ok(())
        }
        Message::BackSpace => {
            app.pop_user_input();
            Ok(())
        }
        Message::Delete => delete_handler(app),
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
        Message::NoOp => Ok(()),
        _ => bail!("Message handler not implemented"),
    }
}
