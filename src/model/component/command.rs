use eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    Frame,
};

use crate::model::app::App;

use super::{popup::Popup, StrataComponent};

type CommandFunction = Box<dyn FnOnce(&str, &mut App) -> Result<()>>;

pub struct AppCommand {
    command_name: String,
    input: String,
    function: CommandFunction,
}

impl Default for AppCommand {
    fn default() -> Self {
        Self {
            command_name: String::new(),
            input: String::new(),
            function: Box::new(|_, _| Ok(())),
        }
    }
}

impl AppCommand {
    pub fn new(command_name: &str, input: &str, function: CommandFunction) -> AppCommand {
        AppCommand {
            command_name: command_name.to_string(),
            input: input.to_string(),
            function,
        }
    }

    pub fn command_name(&self) -> &str {
        &self.command_name
    }

    pub fn execute(self, app: &mut App) -> Result<()> {
        (self.function)(&self.input, app)
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

impl StrataComponent for AppCommand {
    fn render(&self, frame: &mut Frame, area: Rect, _is_focused: bool) {
        let popup = Popup {
            title: self.command_name.clone().into(),
            content: self.input.clone().into(),
            style: Style::default(),
            title_style: Style::new().white().bold(),
            border_style: Style::default(),
        };
        frame.render_widget(popup, area);
    }
}
