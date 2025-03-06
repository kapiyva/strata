#[derive(Debug, Clone, PartialEq)]
pub enum DisplayFocus {
    TableSelector,
    TableView,
    Command(Box<DisplayFocus>),
    Error(Box<DisplayFocus>),
    Exit(Box<DisplayFocus>),
}

impl Default for DisplayFocus {
    fn default() -> Self {
        DisplayFocus::TableSelector
    }
}

impl ToString for DisplayFocus {
    fn to_string(&self) -> String {
        match self {
            DisplayFocus::TableSelector => "TableList".to_string(),
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
            DisplayFocus::TableSelector => {
                "<a> Add new table | <o> Open CSV file | <d> Delete table | <q> Quit app"
                    .to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_focus_last_focus() {
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::TableSelector),
            DisplayFocus::TableSelector
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::TableView),
            DisplayFocus::TableView
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::Command(Box::new(
                DisplayFocus::TableSelector
            ))),
            DisplayFocus::TableSelector
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::Error(Box::new(DisplayFocus::TableSelector))),
            DisplayFocus::TableSelector
        );
        assert_eq!(
            DisplayFocus::last_focus(&DisplayFocus::Exit(Box::new(DisplayFocus::TableSelector))),
            DisplayFocus::TableSelector
        );
    }
}
