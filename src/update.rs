mod handler;

use crate::{app::App, error::StrataError, message::Message};
use eyre::{bail, OptionExt, Result};
use handler::{
    handle_add::handle_add_table,
    handle_cancel::handle_cancel,
    handle_edit_cell::handle_edit_cell,
    handle_edit_header::handle_edit_header,
    handle_edit_table_name::handle_edit_table_name,
    handle_jump_cell::handle_jump_cell,
    handle_jump_table::handle_jump_table,
    handle_move_cursor::handle_move_cursor,
    handle_open::handle_open,
    handle_save::handle_save,
    handle_table_size::{
        handle_collapse_col, handle_collapse_row, handle_expand_col, handle_expand_row,
    },
};

pub fn update(app: &mut App, message: Message) -> Result<&mut App> {
    match message {
        Message::AddTable => handle_add_table(app),
        Message::PopInput => {
            app.command_mut()
                .ok_or_eyre(StrataError::CommandNotFound)?
                .pop();
            Ok(app)
        }
        Message::Cancel => handle_cancel(app),
        Message::CollapseColumn => handle_collapse_col(app),
        Message::CollapseRow => handle_collapse_row(app),
        Message::RemoveTable => app.remove_table(),
        Message::DeleteCell => {
            let tv = app.selected_table_view_mut()?;
            let (row, col) = tv
                .selected_index()
                .ok_or_else(|| eyre::eyre!("No cell selected"))?;

            tv.update_cell(row, col, "")?;
            Ok(app)
        }
        Message::EditTableName => handle_edit_table_name(app),
        Message::EditCell => handle_edit_cell(app),
        Message::ExecuteCommand => app.execute_command(),
        Message::Exiting => Ok(app.focus_exit()),
        Message::ExpandColumn => handle_expand_col(app),
        Message::ExpandRow => handle_expand_row(app),
        Message::EditHeader => handle_edit_header(app),
        Message::Input(c) => {
            app.command_mut()
                .ok_or_eyre(StrataError::CommandNotFound)?
                .input(c);
            Ok(app)
        }
        Message::JumpTable => handle_jump_table(app),
        Message::JumpCell => handle_jump_cell(app),
        Message::Move(direction) => handle_move_cursor(app, direction),
        Message::NoOp => Ok(app),
        Message::Open => handle_open(app),
        Message::Save => handle_save(app),
        Message::SelectTable => app.focus_table_view(),
        _ => bail!("Message handler not implemented"),
    }
}
