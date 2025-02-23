use std::path::PathBuf;

pub enum Message {
    AddTableMode,
    SelectTableMode,
    SelectCellMode,
    EditHeaderMode,
    EditCellMode,
    Input(char),
    PopInput,
    NewTable(String),
    OpenCsv(PathBuf),
    Move(MoveDirection),
    SelectTable,
    RemoveTable,
    SaveTable(String),
    ExpandRow,
    CollapseRow,
    ExpandColumn,
    CollapseColumn,
    SaveHeader(String),
    SaveCellValue(String),
    Exiting,
    CancelExit,
    Exit,
    NoOp,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
