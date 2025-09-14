use crate::DataId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Workbook error: {0}")]
    Workbook(#[from] calamine::OdsError),

    #[error("Sheet error: {0}")]
    Sheet(#[from] calamine::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Missing link for {type_name}({id})")]
    MissingLink { type_name: &'static str, id: DataId },

    #[error("{type_name} is already loaded")]
    AlreadyLoaded { type_name: &'static str },

    #[error("Duplicate id on {type_name}({id})")]
    DuplicateId { type_name: &'static str, id: DataId },

    #[error("Data out of range: expected {expected}, got {actual}")]
    OutOfRange { expected: usize, actual: usize },

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}
