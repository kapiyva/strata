mod render_error;
mod render_exit;
mod render_footer;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use render_error::render_error;
use render_exit::render_exit;
use render_footer::{render_footer, RenderFooterProps};

use crate::model::{
    app::{state::DisplayFocus, App},
    component::StrataComponent,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // layout
    let [main_area, footer_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(frame.area());
    let [table_selector_area, table_area] =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)])
            .margin(1)
            .areas(main_area);

    // render
    app.get_table_selector().render(
        frame,
        table_selector_area,
        *app.get_display_focus() == DisplayFocus::TableSelector,
    );
    if let Ok(tv) = app.get_selected_table_view() {
        tv.render(
            frame,
            table_area,
            app.get_display_focus() == &DisplayFocus::TableView,
        );
    };
    render_footer(
        frame,
        footer_area,
        RenderFooterProps {
            display_focus: app.get_display_focus(),
        },
    );

    // render overlay
    match app.get_display_focus() {
        DisplayFocus::Command(_) => {
            let command_area = Rect {
                x: frame.area().width / 4,
                y: frame.area().height / 3,
                width: frame.area().width / 2,
                height: 3,
            };
            if let Some(command) = app.get_command() {
                command.render(frame, command_area, true);
            };
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
