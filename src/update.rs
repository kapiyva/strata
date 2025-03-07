mod handler;

use crate::{app::App, error::StrataError, message::Message};
use eyre::{bail, OptionExt, Result};
use handler::{
    handle_add::handle_add,
    handle_cancel::handle_cancel,
    handle_delete::handle_delete,
    handle_edit::handle_edit,
    handle_enter::handle_enter,
    handle_hyper_edit::handle_hyper_edit,
    handle_jump::handle_jump,
    handle_move_cursor::handle_move_cursor,
    handle_open::handle_open,
    handle_save::handle_save,
    handle_table_size::{
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
        Message::Save => handle_save(app),
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
