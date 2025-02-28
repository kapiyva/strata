use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrataError {
    #[error("Invalid operation was called:  operation:[{operation:?}], focus:[{focus:?}]")]
    InvalidOperationCall { operation: String, focus: String },

    #[error("Table already exists: table_name:[{0}]")]
    TableNameDuplicate(String),

    #[error("Table not found: table_name:[{0}]")]
    TableNotFound(String),

    #[error("No table added")]
    NoTableAdded,

    #[error("No table selected")]
    NoTableSelected,

    #[error("Invalid table name")]
    InvalidTableName,

    #[error("Invalid row index: max:[{max}], requested:[{requested}]")]
    InvalidRowIndex { max: usize, requested: usize },

    #[error("Invalid column index: max:[{max}], requested:[{requested}]")]
    InvalidColumnIndex { max: usize, requested: usize },

    #[error("No cell selected")]
    NoCellSelected,

    #[error("Command not found")]
    CommandNotFound,

    #[error("String parse failed: {0}")]
    StringParseError(String),
}
