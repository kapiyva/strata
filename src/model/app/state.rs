#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DisplayMode {
    AddTable,
    SelectTable,
    SelectCell,
    EditCell,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::SelectTable
    }
}

impl ToString for DisplayMode {
    fn to_string(&self) -> String {
        match self {
            DisplayMode::AddTable => "AddTable".to_string(),
            DisplayMode::SelectTable => "SelectTable".to_string(),
            DisplayMode::SelectCell => "DisplayTable".to_string(),
            DisplayMode::EditCell => "EditCell".to_string(),
        }
    }
}
