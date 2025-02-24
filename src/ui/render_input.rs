use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    Frame,
};

use super::widget::popup::Popup;

pub(crate) fn render_input(frame: &mut Frame, input: &str, title: &str) {
    let popup_area = Rect {
        x: frame.area().width / 4,
        y: frame.area().height / 3,
        width: frame.area().width / 2,
        height: 3,
    };
    let popup = Popup {
        title: title.into(),
        content: input.into(),
        style: Style::default(),
        title_style: Style::new().white().bold(),
        border_style: Style::default(),
    };
    frame.render_widget(popup, popup_area);
}
