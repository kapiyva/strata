mod handler;

use crate::{error::StrataError, message::Message, model::app::App};
use eyre::{bail, OptionExt, Result};
use handler::{
    add_handler::handle_add,
    cancel_handler::handle_cancel,
    delete_handler::handle_delete,
    edit_handler::handle_edit,
    enter_handler::handle_enter,
    hyper_edit_handler::handle_hyper_edit,
    jump_handler::handle_jump,
    move_cursor_handler::handle_move_cursor,
    open_handler::handle_open,
    table_size_handler::{
        handle_collapse_col, handle_collapse_row, handle_expand_col, handle_expand_row,
    },
};

pub fn update(app: &mut App, message: Message) -> Result<()> {
    match message {
        Message::Enter => handle_enter(app),
        Message::Cancel => handle_cancel(app),
        Message::Exiting => {
            app.focus_exit();
            Ok(())
        }
        Message::Move(direction) => handle_move_cursor(app, direction),
        Message::Jump => handle_jump(app),
        Message::Add => handle_add(app),
        Message::Open => handle_open(app),
        Message::Edit => handle_edit(app),
        Message::HyperEdit => handle_hyper_edit(app),
        Message::Input(c) => {
            app.command_mut()
                .ok_or_eyre(StrataError::CommandNotFound)?
                .input(c);
            Ok(())
        }
        Message::BackSpace => {
            app.command_mut()
                .ok_or_eyre(StrataError::CommandNotFound)?
                .pop();
            Ok(())
        }
        Message::Delete => handle_delete(app),
        Message::ExpandRow => handle_expand_row(app),
        Message::CollapseRow => handle_collapse_row(app),
        Message::ExpandColumn => handle_expand_col(app),
        Message::CollapseColumn => handle_collapse_col(app),
        Message::NoOp => Ok(()),
        _ => bail!("Message handler not implemented"),
    }
}
