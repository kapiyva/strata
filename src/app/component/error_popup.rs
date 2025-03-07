use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    Frame,
};

use crate::app::base_component::popup::Popup;

use super::{component_style, StrataPopup};

#[derive(Default)]
pub struct ErrorPopup {
    error_message: Vec<String>,
}

impl ErrorPopup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_error_message(&self) -> &Vec<String> {
        &self.error_message
    }

    pub fn is_empty(&self) -> bool {
        self.error_message.is_empty()
    }

    pub fn size(&self) -> usize {
        self.error_message.len()
    }

    pub fn push(&mut self, message: String) -> &mut Self {
        self.error_message.push(message);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.error_message.clear();
        self
    }
}

impl StrataPopup for ErrorPopup {
    fn render(&self, frame: &mut Frame) {
        let area = Rect {
            x: frame.area().width / 4,
            y: frame.area().height / 3,
            width: frame.area().width / 2,
            height: self.error_message.len() as u16 + 6,
        };
        let popup = Popup {
            title: "Error".into(),
            content: self.error_message.join("\n").into(),
            style: component_style(true),
            title_style: Style::new().white().bold(),
            border_style: Style::default(),
        };
        frame.render_widget(popup, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_popup() {
        let mut error_popup = ErrorPopup::new();
        error_popup.push("Error message".into());
        assert!(!error_popup.is_empty());
        assert_eq!(error_popup.size(), 1);

        error_popup.clear();
        assert!(error_popup.is_empty());
        assert_eq!(error_popup.size(), 0);
    }
}
