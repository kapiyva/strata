use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    Frame,
};

use crate::model::app::state::DisplayFocus;

pub struct RenderFooterProps<'a> {
    pub display_mode: &'a DisplayFocus,
}

pub(super) fn render_footer(frame: &mut Frame, area: Rect, props: RenderFooterProps) {
    let footer = Line::from(format!("{}", props.display_mode.get_guide()))
        .style(Style::default().fg(Color::LightCyan));
    frame.render_widget(footer, area);
}
