mod render_exit;
mod render_footer;

use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};
use render_exit::render_exit;
use render_footer::render_footer;

use crate::app::{
    component::{StrataComponent, StrataPopup},
    display_focus::DisplayFocus,
    App,
};

pub fn view(frame: &mut Frame, app: &App) {
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
    
    match app.display_focus() {
        DisplayFocus::FileView => {
            if let Some(file_view) = app.file_view() {
                file_view.render(frame, table_area, true);
            }
        }
        _ => {
            if let Ok(tv) = app.selected_table_view() {
                tv.render(
                    frame,
                    table_area,
                    app.display_focus() == &DisplayFocus::TableView,
                );
            }
        }
    }
    render_footer(frame, footer_area, app.display_focus());

    // render overlay
    match app.display_focus() {
        DisplayFocus::Command(_) => {
            if let Some(command) = app.command() {
                command.render(frame);
            };
        }
        DisplayFocus::Error(_) => {
            app.error_popup().render(frame);
        }
        DisplayFocus::Exit(_) => {
            render_exit(frame);
        }
        _ => {}
    }
}
