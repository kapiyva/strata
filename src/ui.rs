mod render_exit;
mod render_footer;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use render_exit::render_exit;
use render_footer::render_footer;

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
    app.table_selector().render(
        frame,
        table_selector_area,
        *app.display_focus() == DisplayFocus::TableSelector,
    );
    if let Ok(tv) = app.selected_table_view() {
        tv.render(
            frame,
            table_area,
            app.display_focus() == &DisplayFocus::TableView,
        );
    };
    render_footer(frame, footer_area, app.display_focus());

    // render overlay
    match app.display_focus() {
        DisplayFocus::Command(_) => {
            let command_area = Rect {
                x: frame.area().width / 4,
                y: frame.area().height / 3,
                width: frame.area().width / 2,
                height: 3,
            };
            if let Some(command) = app.command() {
                command.render(frame, command_area, true);
            };
        }
        DisplayFocus::Error(_) => {
            let error_area = Rect {
                x: frame.area().width / 4,
                y: frame.area().height / 3,
                width: frame.area().width / 2,
                height: app.error_popup().size() as u16 + 6,
            };
            // render_error(frame, error_area, app.get_error_message());
            app.error_popup().render(frame, error_area, true);
        }
        DisplayFocus::Exit(_) => {
            render_exit(frame);
        }
        _ => {}
    }
}
