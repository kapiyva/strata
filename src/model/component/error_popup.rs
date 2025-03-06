use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    Frame,
};

use super::{popup::Popup, StrataComponent};

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

impl StrataComponent for ErrorPopup {
    fn render(&self, frame: &mut Frame, area: Rect, _is_focused: bool) {
        let popup = Popup {
            title: "Error".into(),
            content: self.error_message.join("\n").into(),
            style: Style::default(),
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
