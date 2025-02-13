use std::path::PathBuf;

use crate::model::table::TableName;

pub enum Message {
    NewTable(TableName),
    OpenFile(PathBuf),
    OpenTable(TableName),
    SaveTable(TableName),
}
