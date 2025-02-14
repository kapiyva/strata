use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrataError {
    #[error("Invalid operation was called:  operation:[{operation:?}], state:[{state:?}]")]
    InvalidOperationCall { operation: String, state: String },
    #[error("Table already exists: table_name:[{0}]")]
    TableDuplicate(String),
    #[error("Table not found: table_name:[{0}]")]
    TableNotFound(String),
    #[error("No table selected")]
    NoTableSelected,
    #[error("Invalid table name")]
    InvalidTableName,
    #[error("Invalid row index: max:[{max}], requested:[{requested}]")]
    InvalidRowIndex { max: usize, requested: usize },
    #[error("Invalid column index: max:[{max}], requested:[{requested}]")]
    InvalidColumnIndex { max: usize, requested: usize },
}
