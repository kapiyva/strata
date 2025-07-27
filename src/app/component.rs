pub mod command;
pub mod error_popup;
pub mod file_view;
pub mod table_selector;
pub mod table_view;

use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    Frame,
};

pub trait StrataComponent {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool);
}

pub trait StrataPopup {
    fn render(&self, frame: &mut Frame);
}

pub fn component_style(is_focused: bool) -> Style {
    if is_focused {
        Style::default().bold().fg(Color::LightYellow)
    } else {
        Style::default()
    }
}

pub fn selectable_item_style_factory(is_focused: bool) -> impl Fn(bool) -> Style {
    move |is_selected: bool| -> Style {
        if !is_selected {
            return Style::default();
        }
        if is_focused {
            Style::default().bg(Color::LightBlue)
        } else {
            Style::default().bg(Color::DarkGray)
        }
    }
}
