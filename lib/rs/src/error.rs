use crate::DataId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Workbook error: {0}")]
    Workbook(#[from] calamine::OdsError),

    #[error("Sheet error: {0}")]
    Sheet(#[from] calamine::Error),

    #[error("Parse error [{workbook} / {sheet} / {row} / {column}]: {error}")]
    Parse {
        workbook: &'static str,
        sheet: &'static str,
        row: usize,
        column: &'static str,
        error: ParseError
    },

    #[error("Missing link for {type_name}({id})")]
    MissingLink { type_name: &'static str, id: DataId },

    #[error("Duplicate id on {type_name}({id}): {a}, {b}")]
    DuplicateId { type_name: &'static str, id: DataId, a: String, b: String },

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid format [{type_name}]: expected {expected}, got {actual}")]
    InvalidFormat { type_name: &'static str, expected: &'static str, actual: String },

    #[error("Invalid value [{type_name}]: {value}")]
    InvalidValue { type_name: &'static str, value: String },

    #[error("Out of range [{type_name}]: expected [{min}, {max}], got {value}")]
    OutOfRange { type_name: &'static str, min: i64, max: i64, value: i64 },

    #[error("Invalid columns count: e")]
    InvalidColumnCount { expected: usize, actual: usize },

    #[error("Invalid item count [{type_name}]: expected {expected}, got {actual}")]
    InvalidItemCount { type_name: &'static str, expected: usize, actual: usize },
}
