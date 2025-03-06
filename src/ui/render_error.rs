use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    Frame,
};

use crate::model::component::popup::Popup;

pub(crate) fn render_error(frame: &mut Frame, error_message: &Vec<String>) {
    let popup_area = Rect {
        x: frame.area().width / 4,
        y: frame.area().height / 3,
        width: frame.area().width / 2,
        height: error_message.len() as u16 + 6,
    };
    let popup = Popup {
        title: "Error".into(),
        content: error_message.join("\n").into(),
        style: Style::default(),
        title_style: Style::new().white().bold(),
        border_style: Style::default(),
    };
    frame.render_widget(popup, popup_area);
}
