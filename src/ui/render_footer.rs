use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    Frame,
};

use crate::model::app::state::DisplayFocus;

pub(super) fn render_footer(frame: &mut Frame, area: Rect, focus: &DisplayFocus) {
    let footer =
        Line::from(format!("{}", focus.get_guide())).style(Style::default().fg(Color::LightCyan));
    frame.render_widget(footer, area);
}
