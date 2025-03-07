use eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    Frame,
};

use crate::app::{base_component::popup::Popup, App};

use super::{component_style, StrataPopup};

type Command = Box<dyn FnOnce(&str, &mut App) -> Result<()>>;

pub struct CommandPopup {
    title: String,
    input: String,
    command: Command,
}

impl Default for CommandPopup {
    fn default() -> Self {
        Self {
            title: String::new(),
            input: String::new(),
            command: Box::new(|_, _| Ok(())),
        }
    }
}

impl CommandPopup {
    pub fn new(command_name: &str, input: &str, function: Command) -> CommandPopup {
        CommandPopup {
            title: command_name.to_string(),
            input: input.to_string(),
            command: function,
        }
    }

    pub fn command_name(&self) -> &str {
        &self.title
    }

    pub fn execute(self, app: &mut App) -> Result<()> {
        (self.command)(&self.input, app)
    }

    pub fn input(&mut self, c: char) -> &mut Self {
        self.input.push(c);
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.input.pop();
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.input.clear();
        self
    }
}

impl StrataPopup for CommandPopup {
    fn render(&self, frame: &mut Frame) {
        let area = Rect {
            x: frame.area().width / 4,
            y: frame.area().height / 3,
            width: frame.area().width / 2,
            height: 3,
        };
        let popup = Popup {
            title: self.title.clone().into(),
            content: self.input.clone().into(),
            style: component_style(true),
            title_style: Style::new().white().bold(),
            border_style: Style::default(),
        };
        frame.render_widget(popup, area);
    }
}
