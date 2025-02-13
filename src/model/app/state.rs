use crate::model::table::TableName;

const DEFAULT_CELL_INDEX: usize = 0;

#[derive(Debug)]
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
        })
    }
}

#[derive(Debug)]
pub struct AddTableState {
    pub selected_cell: Option<SelectedCell>,
}

#[derive(Debug)]
pub struct SelectTableState {
    pub selected_cell: Option<SelectedCell>,
}

#[derive(Debug)]
pub struct DisplayTableState {
    pub selected_cell: SelectedCell,
}

#[derive(Debug)]
pub struct EditCellState {
    pub selected_cell: SelectedCell,
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
