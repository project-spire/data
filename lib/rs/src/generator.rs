use std::collections::HashMap;
use std::fmt::Formatter;
use crate::generator::table::{Table, TableKind};

pub mod table;

const TAB: &str = "    ";

#[derive(Debug)]
pub enum GenerateError {
    DuplicatedTableName,
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::DuplicatedTableName => write!(f, "Duplicated table name"),
        }
    }
}

impl std::error::Error for GenerateError {}

pub fn register_table_types(
    tables: &Vec<Table>,
    table_types: &mut HashMap<String, TableKind>,
) -> Result<(),GenerateError > {
    for table in tables {
        if table_types.contains_key(&table.name) {
            return Err(GenerateError::DuplicatedTableName);
        }
    }

    Ok(())
}

pub fn generate_table_code(
    table: &Table,
    table_types: &HashMap<String, TableKind>,
) -> Result<String, GenerateError> {
    match table.kind {
        TableKind::Concrete => generate_concrete_table_code(table, table_types),
        TableKind::Abstract => generate_abstract_table_code(table, table_types),
    }
}

fn generate_concrete_table_code(
    table: &Table,
    _table_types: &HashMap<String, TableKind>,
) -> Result<String, GenerateError> {
    let table_name = &table.name;
    let fields_code = table
        .fields
        .iter()
        .map(|field| {
            format!("{TAB}pub {}: {},", field.name, field.kind.to_rust_type())
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!(
r#"
#[derive(Debug, serde::Deserialize)]
pub struct {table_name} {{
{fields_code}
}}

impl {table_name} {{
    pub fn load() {{
        unimplemented!()
    }}
"#
    ))
}

fn generate_abstract_table_code(
    table: &Table,
    table_types: &HashMap<String, TableKind>,
) -> Result<String, GenerateError> {
    todo!()
}