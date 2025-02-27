pub enum Message {
    Enter,
    Cancel,
    Exiting,
    Exit,
    Move(MoveDirection),
    Jump,
    Add,
    Open,
    Edit,
    HyperEdit,
    Input(char),
    BackSpace,
    Delete,
    ExpandRow,
    CollapseRow,
    ExpandColumn,
    CollapseColumn,
    NoOp,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
