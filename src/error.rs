use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrataError {
    #[error("Command not found")]
    CommandNotFound,

    #[error("Failed to get file name for path: {0}")]
    FailedToReadDir(String),

    #[error("Index out of bounds: max:[{max}], requested:[{requested}]")]
    IndexOutOfBounds { max: usize, requested: usize },

    #[error("Invalid column index: max:[{max}], requested:[{requested}]")]
    InvalidColumnIndex { max: usize, requested: usize },

    #[error("Invalid operation was called:  operation:[{operation:?}], focus:[{focus:?}]")]
    InvalidOperationCall { operation: String, focus: String },

    #[error("Invalid row index: max:[{max}], requested:[{requested}]")]
    InvalidRowIndex { max: usize, requested: usize },

    #[error("Invalid table name")]
    InvalidTableName,

    #[error("Item not found: item_name:[{0}]")]
    ItemNotFound(String),

    #[error("No cell selected")]
    NoCellSelected,

    #[error("No item selected")]
    NoItemSelected,

    #[error("No table added")]
    NoTableAdded,

    #[error("No table selected")]
    NoTableSelected,

    #[error("String parse failed: {0}")]
    StringParseError(String),

    #[error("Table Has No Header")]
    TableHasNoHeader,

    #[error("Table already exists: table_name:[{0}]")]
    TableNameDuplicate(String),

    #[error("Table not found: table_name:[{0}]")]
    TableNotFound(String),
}
