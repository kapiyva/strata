mod component;
mod render_error;
mod render_exit;
mod render_footer;
mod render_input;
mod render_table;
mod render_table_list;

use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};
use render_error::render_error;
use render_exit::render_exit;
use render_footer::{render_footer, RenderFooterProps};
use render_input::render_command_box;
use render_table::{render_table, RenderTableProps};
use render_table_list::{render_table_list, RenderTableListProps};

use crate::model::app::{state::DisplayFocus, App};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // layout
    let [main_area, footer_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(frame.area());
    let [table_list_area, table_area] =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)])
            .margin(1)
            .areas(main_area);
    //render
    render_table_list(
        frame,
        table_list_area,
        RenderTableListProps {
            table_list: app.get_table_name_list(),
            selected_index: app.get_table_selector(),
            focused: *app.get_display_focus() == DisplayFocus::TableList,
        },
    );
    if let Ok(table) = app.get_selected_table_data() {
        render_table(
            frame,
            table_area,
            RenderTableProps {
                table,
                focused: *app.get_display_focus() == DisplayFocus::TableView,
            },
        );
    };
    render_footer(
        frame,
        footer_area,
        RenderFooterProps {
            display_focus: app.get_display_focus(),
        },
    );
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            render_command_box(
                frame,
                app.get_user_input(),
                app.get_command_name().unwrap_or("Command"),
            );
        }
        DisplayFocus::Error(_) => {
            render_error(frame, app.get_error_message());
        }
        DisplayFocus::Exit(_) => {
            render_exit(frame);
        }
        _ => {}
    }
}
