#[derive(Debug, PartialEq, Eq)]
pub enum DisplayMode {
    AddTable,
    SelectTable,
    SelectCell,
    EditHeader,
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
            DisplayMode::EditHeader => "EditHeader".to_string(),
            DisplayMode::EditCell => "EditCell".to_string(),
        }
    }
}
