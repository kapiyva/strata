use crate::model::table::TableName;
const DEFAULT_CELL_INDEX: usize = 0;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DisplayState {
    AddTable(AddTableState),
    SelectTable(SelectTableState),
    DisplayTable(DisplayTableState),
    EditCell(EditCellState),
}

impl Default for DisplayState {
    fn default() -> Self {
        DisplayState::SelectTable(SelectTableState {
            selected_cell: None,
            table_list: Vec::new(),
            cursor: 0,
        })
    }
}

impl ToString for DisplayState {
    fn to_string(&self) -> String {
        match self {
            DisplayState::AddTable(_) => "AddTable".to_string(),
            DisplayState::SelectTable(_) => "SelectTable".to_string(),
            DisplayState::DisplayTable(_) => "DisplayTable".to_string(),
            DisplayState::EditCell(_) => "EditCell".to_string(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AddTableState {
    pub selected_cell: Option<SelectedCell>,
    pub table_list: Vec<TableName>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SelectTableState {
    pub selected_cell: Option<SelectedCell>,
    pub table_list: Vec<TableName>,
    pub cursor: usize,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DisplayTableState {
    pub selected_cell: SelectedCell,
    pub table_list: Vec<TableName>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EditCellState {
    pub selected_cell: SelectedCell,
    pub table_list: Vec<TableName>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SelectedCell {
    pub table_name: TableName,
    pub row: usize,
    pub col: usize,
}

impl SelectedCell {
    pub fn new(table_name: TableName) -> Self {
        Self {
            table_name,
            row: DEFAULT_CELL_INDEX,
            col: DEFAULT_CELL_INDEX,
        }
    }
}
