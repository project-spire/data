use heck::ToSnakeCase;
use std::collections::HashMap;
use std::fmt::Formatter;
use crate::generator::table::{FieldKind, Table, TableKind};

pub mod table;

const TAB: &str = "    ";

#[derive(Debug)]
pub enum GenerateError {
    DuplicatedTableName { table_name: String },
    UnknownTableName { table_name: String },
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::DuplicatedTableName { table_name } =>
                write!(f, "Duplicated table name {}", table_name),
            GenerateError::UnknownTableName {table_name} =>
                write!(f, "Unknown table name {}", table_name),
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
            return Err(GenerateError::DuplicatedTableName { table_name: table.name.clone() });
        }

        table_types.insert(table.name.clone(), table.kind);
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
    table_types: &HashMap<String, TableKind>,
) -> Result<String, GenerateError> {
    let table_name = &table.name;
    let sheet_name = &table.sheet;
    let data_name = format!("{}Data", table_name);
    let data_cell_name = format!("{}_DATA", table_name.to_snake_case().to_uppercase());

    // Check fields
    let mut has_link = false;
    let mut lifetime_code = String::new();
    let mut lifetime_parameter_code = String::new();

    let mut imports = Vec::new();
    let mut fields = Vec::new();

    for field in &table.fields {
        if let FieldKind::Link { link_type } = &field.kind {
            if !table_types.contains_key(link_type) {
                println!("{:?}", table_types);
                return Err(GenerateError::UnknownTableName { table_name: table_name.clone() });
            }

            has_link = true;
            lifetime_code = "<'a>".to_string();
            lifetime_parameter_code = "<'_>".to_string();

            imports.push(format!("use crate::data::{};", link_type));
        }

        fields.push(format!("{TAB}pub {}: {},", field.name, field.kind.to_rust_type()));
    }

    // Generate codes
    let base_imports_code = if has_link {
        "use crate::data::{DataId, Link};"
    } else {
        "use crate::data::DataId;"
    };
    let imports_code = imports.join("\n");
    let fields_code = fields.join("\n");

    Ok(format!(
r#"// Generated file
use tracing::info;
{base_imports_code}
{imports_code}

static {data_cell_name}: tokio::sync::OnceCell<{data_name}> = tokio::sync::OnceCell::const_new();

#[derive(Debug, serde::Deserialize)]
pub struct {table_name}{lifetime_code} {{
{fields_code}
}}

impl{lifetime_code} {table_name}{lifetime_parameter_code} {{
    pub fn load(
        reader: &mut calamine::Ods<std::io::BufReader<std::fs::File>>,
    ) -> Result<(), calamine::Error> {{
        let range = reader.worksheet_range("{sheet_name}")?;
        for row in range.rows() {{
            todo!()
        }}

        info!("Loaded {{}} rows", range.len());
        Ok(())
    }}
}}

pub struct {data_name}{lifetime_code} {{
    pub data: std::collections::HashMap<DataId, {table_name}{lifetime_code}>,
}}

impl{lifetime_code} {data_name}{lifetime_parameter_code} {{
    pub fn new() -> Self {{
        Self {{
            data: std::collections::HashMap::new()
        }}
    }}
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