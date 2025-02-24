mod render_footer;
mod render_input;
mod render_table;
mod render_table_list;
mod widget;

use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};
use render_footer::render_footer;
use render_input::render_input;
use render_table::render_table;
use render_table_list::render_table_list;

use crate::model::app::{state::DisplayMode, App};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let [main_area, footer_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(frame.area());
    let [table_list_area, table_area] =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Min(0)])
            .margin(1)
            .areas(main_area);

    render_table_list(frame, table_list_area, app);
    let _ = render_table(frame, table_area, app);
    let _ = render_footer(frame, footer_area, app);
    match app.get_display_mode() {
        DisplayMode::AddTable => {
            render_input(frame, app.get_user_input(), "Table Name");
        }
        DisplayMode::EditHeader => {
            render_input(frame, app.get_user_input(), "Header Name");
        }
        DisplayMode::EditCell => {
            render_input(frame, app.get_user_input(), "Cell Value");
        }
        _ => {}
    }
}
