use ratatui::{layout::Rect, Frame};

pub mod command;
// pub mod error;
pub mod popup;
pub mod table_selector;
pub mod table_view;

pub trait StrataComponent {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool);
}
