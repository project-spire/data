use std::collections::VecDeque;
use std::fs;
use crate::*;
use crate::generator::*;

const TUPLE_TYPES_MAX_COUNT: usize = 4;

#[derive(Debug)]
pub struct TableEntry {
    pub name: Name,
    pub schema: TableSchema,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TableSchema {
    Concrete(ConcreteTableSchema),
    Abstract(AbstractTableSchema),
}

#[derive(Debug, Deserialize)]
pub struct ConcreteTableSchema {
    pub name: String,
    pub data: String,
    pub sheet: String,
    pub fields: Vec<Field>,
    pub extend: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AbstractTableSchema {
    pub name: String,
    pub fields: Vec<Field>,
    pub extend: Option<String>,
}

pub trait TableSchematic {
    fn name(&self) -> &str;
    fn fields(&self) -> &Vec<Field>;
    fn extend(&self) -> &Option<String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(flatten)] pub kind: FieldKind,
    #[serde(default)] pub desc: Option<String>,
    #[serde(default)] pub optional: Option<bool>,
    #[serde(default)] pub cardinality: Option<Cardinality>,
    #[serde(default)] pub constraints: Option<Vec<Constraint>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum FieldKind {
    Scalar {
        #[serde(rename = "type")] scalar_type: ScalarAllType
    },
    Enum {
        #[serde(rename = "type")] enum_type: String
    },
    Link {
        #[serde(rename = "type")] link_type: String
    },
    Tuple {
        types: Vec<FieldKind>,
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScalarAllType {
    Id,
    Bool,
    Int8, Int16, Int32, Int64,
    Uint8, Uint16, Uint32, Uint64,
    Float32, Float64,
    String,
    Datetime,
    Duration,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    Single,
    Multi
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Constraint {
    Exclusive
}


impl Generator {
    pub fn generate_table(
        &self,
        table: &TableEntry
    ) -> Result<(), Error> {
        let table_file = self
            .full_gen_dir(&table.name.parent_namespace())
            .join(format!("{}.rs", table.name.as_entity()));
        self.log(&format!("Generating table `{}`", table_file.display()));

        let code = match &table.schema {
            TableSchema::Concrete(schema) => {
                self.generate_concrete_table(&table.name, schema)?
            }
            TableSchema::Abstract(schema) => {
                self.generate_abstract_table(&table.name, schema)?
            }
        };

        fs::write(table_file, code)?;

        Ok(())
    }
    
    fn generate_concrete_table(
        &self,
        name: &Name,
        schema: &ConcreteTableSchema,
    ) -> Result<String, Error> {
        // Prepare field codes
        let mut has_lifetime = false;
        let mut lifetime_code = String::new();
        let mut lifetime_parameter_code = String::new();

        let mut field_names = Vec::new();
        let mut field_parses = Vec::new();
        let mut field_definitions = Vec::new();

        let fields = self.get_table_all_fields(schema)?;
        for (index, field) in fields.iter().enumerate() {
            if let FieldKind::Link { .. } = &field.kind {
                has_lifetime = true;
            }

            field_names.push(field.name.clone());
            field_definitions.push(format!("{TAB}pub {}: {},", field.name, field.kind.to_rust_type()));
            field_parses.push(format!(
                "{TAB}{TAB}let {field_name} = {parse_function}(&row[{index}])?;",
                field_name = field.name,
                parse_function = field.kind.to_parse_code(),
            ));
        }

        if has_lifetime {
            lifetime_code = "<'a>".to_string();
            lifetime_parameter_code = "<'_>".to_string();
        }

        let fields_count = fields.len();

        // Generate codes
        let data_cell_name = name.as_data_type_cell();
        let data_type_name = name.as_data_type();
        let table_type_name = name.as_type(false);
        
        let field_definitions_code = field_definitions.join("\n");
        let field_parses_code = field_parses.join("\n");
        let field_passes_code = field_names
            .iter()
            .map(|name| {
                format!("{TAB}{TAB}{TAB}{name},")
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!(
r#"{GENERATED_FILE_WARNING}
use std::collections::HashMap;
use tracing::info;
use {CRATE_PREFIX}::{{DataId, error::Error}};

static {data_cell_name}: tokio::sync::OnceCell<{data_type_name}> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub struct {table_type_name}{lifetime_code} {{
{field_definitions_code}
}}

impl {table_type_name}{lifetime_code} {{
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), Error> {{
        const FIELDS_COUNT: usize = {fields_count};

        if row.len() != FIELDS_COUNT {{
            return Err(Error::OutOfRange {{ expected: FIELDS_COUNT, actual: row.len() }});
        }}

{field_parses_code}

        Ok((id, Self {{
{field_passes_code}
        }}))
    }}
}}

impl{lifetime_code} {CRATE_PREFIX}::Linkable for {table_type_name}{lifetime_parameter_code} {{
    fn get(id: DataId) -> Option<&'static Self> {{
        {data_type_name}::get(id)
    }}
}}

pub struct {data_type_name}{lifetime_code} {{
    data: HashMap<DataId, {table_type_name}{lifetime_code}>,
}}

impl{lifetime_code} {data_type_name}{lifetime_parameter_code} {{
    pub fn get(id: DataId) -> Option<&'static {table_type_name}> {{
        {data_cell_name}.get().unwrap().data.get(&id)
    }}

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static {table_type_name})> {{
        {data_cell_name}.get().unwrap().data.iter()
    }}
}}

impl{lifetime_code} {CRATE_PREFIX}::Loadable for {data_type_name}{lifetime_parameter_code} {{
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {{
        let mut objects = HashMap::new();
        for row in rows {{
            let (id, object) = {table_type_name}::parse(row)?;

            if objects.contains_key(&id) {{
                return Err(Error::DuplicatedId {{
                    type_name: std::any::type_name::<{table_type_name}>(),
                    id,
                }});
            }}

            objects.insert(id, object);
        }}

        if !{data_cell_name}.set(Self {{ data: objects }}).is_ok() {{
            return Err(Error::AlreadyLoaded {{
                type_name: std::any::type_name::<{table_type_name}>(),
            }});
        }}

        info!("Loaded {{}} rows", rows.len());
        Ok(())
    }}
}}
"#))
    }

    fn generate_abstract_table(
        &self,
        name: &Name,
        schema: &AbstractTableSchema,
    ) -> Result<String, Error> {
        let data_cell_name = name.as_data_type_cell();
        let data_type_name = name.as_data_type();
        let table_type_name = name.as_type(false);

        let mut child_types = Vec::new();
        let mut child_id_matches = Vec::new();
        let mut child_loads = Vec::new();

        for index in &self.table_hierarchies[&self.table_indices[&name.as_type(true)]] {
            let child_name = self.tables[*index].name.as_type(false);
            let child_full_name = self.tables[*index].name.as_type(true);
            let child_full_data_name = child_full_name.clone() + "Data";

            child_types.push(format!(
                "{TAB}{}(&'static {CRATE_PREFIX}::{}),",
                child_name,
                child_full_name,
            ));

            child_id_matches.push(format!(
                "{TAB}{TAB}{TAB}Self::{child_name}(x) => &x.id,"
            ));

            child_loads.push(format!(
r#"        for (id, row) in {CRATE_PREFIX}::{child_full_data_name}::iter() {{
            check(&data, id)?;
            data.insert(*id, {table_type_name}::{child_name}(row));
        }}"#
            ));
        }

        let child_types_code = child_types.join("\n");
        let child_id_matches_code = child_id_matches.join("\n");
        let child_loads_code = child_loads.join("\n");

        Ok(format!(
            r#"{GENERATED_FILE_WARNING}
use std::collections::HashMap;
use tracing::info;
use {CRATE_PREFIX}::{{DataId, error::Error}};

static {data_cell_name}: tokio::sync::OnceCell<{data_type_name}> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub enum {table_type_name} {{
{child_types_code}
}}

pub struct {data_type_name} {{
    data: HashMap<DataId, {table_type_name}>,
}}

impl {table_type_name} {{
    pub fn id(&self) -> &DataId {{
        match self {{
{child_id_matches_code}
        }}
    }}
}}

impl {CRATE_PREFIX}::Loadable for {data_type_name} {{
    fn load(_: &[&[calamine::Data]]) -> Result<(), Error> {{
        fn check(data: &HashMap<DataId, {table_type_name}>, id: &DataId) -> Result<(), Error> {{
            if data.contains_key(id) {{
                return Err(Error::DuplicatedId {{
                    type_name: std::any::type_name::<{table_type_name}>(),
                    id: *id,
                }});
            }}
            Ok(())
        }};

        let mut data = HashMap::new();

{child_loads_code}

        if !{data_cell_name}.set(Self {{ data }}).is_ok() {{
            return Err(Error::AlreadyLoaded {{
                type_name: std::any::type_name::<{table_type_name}>(),
            }});
        }}
        Ok(())
    }}
}}
"#
        ))
    }

    /// Get all fields of table including its parents
    pub fn get_table_all_fields(&self, schema: &dyn TableSchematic) -> Result<Vec<Field>, Error> {
        let mut fields = VecDeque::from(schema.fields().clone());
        let mut extend = schema.extend().clone();

        while let Some(parent) = extend.take() {
            let parent_index = self.table_indices
                .get(&parent)
                .ok_or_else(|| Error::Inheritance(schema.name().to_owned(), parent))?;
            let parent_table = &self.tables[*parent_index];
            let parent_schema = parent_table.schema.schematic();

            for field in parent_schema.fields().iter().rev() {
                fields.push_front(field.clone());
            }

            extend = parent_schema.extend().clone();
        }

        Ok(Vec::from(fields))
    }
}

impl TableSchema {
    pub fn schematic(&self) -> &dyn TableSchematic {
        match self {
            TableSchema::Concrete(c) => c,
            TableSchema::Abstract(a) => a,
        }
    }
}

impl TableSchematic for ConcreteTableSchema {
    fn name(&self) -> &str { &self.name }
    fn fields(&self) -> &Vec<Field> { &self.fields }
    fn extend(&self) -> &Option<String> { &self.extend }
}

impl TableSchematic for AbstractTableSchema {
    fn name(&self) -> &str { &self.name }
    fn fields(&self) -> &Vec<Field> { &self.fields }
    fn extend(&self) -> &Option<String> { &self.extend }
}

impl FieldKind {
    fn to_rust_type(&self) -> String {
        match &self {
            FieldKind::Scalar { scalar_type: t } => match t {
                ScalarAllType::Id => "DataId",
                ScalarAllType::Bool => "bool",
                ScalarAllType::Int8 => "i8",
                ScalarAllType::Int16 => "i16",
                ScalarAllType::Int32 => "i32",
                ScalarAllType::Int64 => "i64",
                ScalarAllType::Uint8 => "u8",
                ScalarAllType::Uint16 => "u16",
                ScalarAllType::Uint32 => "u32",
                ScalarAllType::Uint64 => "u64",
                ScalarAllType::Float32 => "f32",
                ScalarAllType::Float64 => "f64",
                ScalarAllType::String => "String",
                ScalarAllType::Datetime => "chrono::DateTime",
                ScalarAllType::Duration => "chrono::Duration",
            }.to_string(),
            FieldKind::Enum { enum_type } => {
                format!("{CRATE_PREFIX}::{enum_type}")
            },
            FieldKind::Link { link_type } => {
                format!("crate::Link<'static, {CRATE_PREFIX}::{link_type}>")
            },
            FieldKind::Tuple { types } => {
                let type_strings = to_tuple_type_strings(types);
                format!("({})", type_strings.join(", "))
            },
        }
    }

    fn to_parse_code(&self) -> String {
        match self {
            FieldKind::Scalar { scalar_type: t } => format!("{CRATE_PREFIX}::{}", match t {
                ScalarAllType::Id => "parse_id",
                ScalarAllType::Bool => "parse_bool",
                ScalarAllType::Int8 => "parse_i8",
                ScalarAllType::Int16 => "parse_i16",
                ScalarAllType::Int32 => "parse_i32",
                ScalarAllType::Int64 => "parse_i64",
                ScalarAllType::Uint8 => "parse_u8",
                ScalarAllType::Uint16 => "parse_u16",
                ScalarAllType::Uint32 => "parse_u32",
                ScalarAllType::Uint64 => "parse_u64",
                ScalarAllType::Float32 => "parse_f32",
                ScalarAllType::Float64 => "parse_f64",
                ScalarAllType::String => "parse_string",
                ScalarAllType::Datetime => "parse_datetime",
                ScalarAllType::Duration => "parse_duration",
            }),
            FieldKind::Enum { enum_type } => format!("{CRATE_PREFIX}::{enum_type}::parse"),
            FieldKind::Link { link_type } => format!("{CRATE_PREFIX}::parse_link::<{CRATE_PREFIX}::{link_type}>"),
            FieldKind::Tuple { types } => {
                let type_strings = to_tuple_type_strings(types);
                format!(
                    "{CRATE_PREFIX}::parse_tuple_{}::<{}>",
                    type_strings.len(),
                    type_strings.join(", ")
                )
            },
        }
    }
}

fn to_tuple_type_strings(fields: &[FieldKind]) -> Vec<String> {
    let mut type_strings = Vec::new();
    for t in fields {
        if let FieldKind::Tuple { .. } = t {
            panic!("Nested tuples are not supported");
        }

        type_strings.push(t.to_rust_type())
    }

    if type_strings.len() < 2 {
        panic!("Tuples must have at least two fields");
    }
    if type_strings.len() > TUPLE_TYPES_MAX_COUNT {
        panic!("Tuples with more than {} types are not supported", TUPLE_TYPES_MAX_COUNT);
    }

    type_strings
}
