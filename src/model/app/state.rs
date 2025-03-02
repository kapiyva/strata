use eyre::Result;

use super::App;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayFocus {
    TableList,
    TableView,
    Command(Box<DisplayFocus>),
    Error(Box<DisplayFocus>),
    Exit(Box<DisplayFocus>),
}

impl Default for DisplayFocus {
    fn default() -> Self {
        DisplayFocus::TableList
    }
}

impl ToString for DisplayFocus {
    fn to_string(&self) -> String {
        match self {
            DisplayFocus::TableList => "TableList".to_string(),
            DisplayFocus::TableView => "TableView".to_string(),
            DisplayFocus::Command(_) => "Command".to_string(),
            DisplayFocus::Error(_) => "Error".to_string(),
            DisplayFocus::Exit(_) => "Exit".to_string(),
        }
    }
}

impl DisplayFocus {
    pub fn last_focus(focus: &DisplayFocus) -> DisplayFocus {
        match focus {
            DisplayFocus::Command(focus)
            | DisplayFocus::Error(focus)
            | DisplayFocus::Exit(focus) => Self::last_focus(focus),
            focus => focus.clone(),
        }
    }

    pub fn get_guide(&self) -> String {
        match self {
            DisplayFocus::TableList => {
                "<a> Add new table | <d> Delete table | <q> Quit app".to_string()
            }
            DisplayFocus::TableView => {
                "<r> Add new row | <e> Edit cell | <E> Edit header | <d> Delete cell | <J> Jump"
                    .to_string()
            }
            DisplayFocus::Command(_) => "<Enter> Submit | <Esc> Cancel".to_string(),
            DisplayFocus::Error(_) => " <Enter> Exit".to_string(),
            DisplayFocus::Exit(_) => " <Enter> Exit | <Esc> Cancel".to_string(),
        }
    }
}

pub struct AppCommand {
    command_name: String,
    function: Box<dyn FnOnce(&mut App) -> Result<()>>,
}

impl AppCommand {
    pub fn new(
        command_name: &str,
        function: Box<dyn FnOnce(&mut App) -> Result<()>>,
    ) -> AppCommand {
        AppCommand {
            command_name: command_name.to_string(),
            function,
        }
    }

    pub fn get_command_name(&self) -> &str {
        &self.command_name
    }

    pub fn execute(self, app: &mut App) -> Result<()> {
        (self.function)(app)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_focus_last_focus() {
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::TableList),
            DisplayFocus::TableList
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::TableView),
            DisplayFocus::TableView
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::Command(Box::new(DisplayFocus::TableList))),
            DisplayFocus::TableList
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::Error(Box::new(DisplayFocus::TableList))),
            DisplayFocus::TableList
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::Exit(Box::new(DisplayFocus::TableList))),
            DisplayFocus::TableList
        );
    }
}
