use std::fs;
use crate::*;
use crate::generator::*;
use crate::generator::table::TableSchema;

#[derive(Debug)]
pub struct ModuleEntry {
    pub name: Name,
    pub entries: Vec<EntityEntry>,
}

#[derive(Debug)]
pub enum EntityEntry {
    ModuleIndex(usize),
    TableIndex(usize),
    EnumerationIndex(usize),
    ConstantIndex(usize),
}

impl ModuleEntry {
    pub fn new(name: Name) -> Self {
        Self {
            name,
            entries: Vec::new(),
        }
    }
}

impl Generator {
    pub fn generate_module(
        &self,
        module: &ModuleEntry
    ) -> Result<(), Error> {
        let is_base_module = module.name.name.is_empty() && module.name.namespace.is_empty();

        let module_file = if is_base_module {
            self.full_gen_dir(&[]).parent().unwrap().join("data.rs")
        } else {
            self.full_gen_dir(&module.name.parent_namespace()).join(format!("{}.rs", module.name.as_entity()))
        };
        self.log(&format!("Generating module `{}`", module_file.display()));

        let code = if is_base_module {
            let mut code = self.generate_module_code(&module)?;
            code += &self.generate_table_load_code()?;
            code
        } else {
            self.generate_module_code(module)?
        };

        fs::create_dir_all(self.full_gen_dir(&module.name.namespace))?;
        fs::write(module_file, code)?;

        Ok(())
    }

    fn generate_module_code(&self, module: &ModuleEntry) -> Result<String, Error> {
        let mut imports = Vec::new();
        let mut exports = Vec::new();

        for entry in &module.entries { match entry {
            EntityEntry::ModuleIndex(index) => {
                let child = &self.modules[*index];
                imports.push(format!("pub mod {};", child.name.as_entity()));
            },
            EntityEntry::TableIndex(index) => {
                let child = &self.tables[*index];
                let name = &child.name;

                imports.push(format!("pub mod {};", name.as_entity()));
                exports.push(format!("pub use {}::*;", name.as_entity()));
            },
            EntityEntry::ConstantIndex(index) => {
                let child = &self.constants[*index];
                let name = &child.name;

                imports.push(format!("pub mod {};", name.as_entity()));
                exports.push(format!("pub use {}::*;", name.as_entity()));
            },
            EntityEntry::EnumerationIndex(index) => {
                let child = &self.enumerations[*index];
                let name = &child.name;

                imports.push(format!("pub mod {};", name.as_entity()));
                exports.push(format!("pub use {}::*;", name.as_entity()));
            },
        } }

        let imports_code = imports.join("\n");
        let exports_code = exports.join("\n");

        Ok(format!(
r#"{GENERATED_FILE_WARNING}
{imports_code}

{exports_code}
"#
        ))
    }

    fn generate_table_load_code(&self) -> Result<String, Error> {
        let mut level_handles = Vec::new();
        for (level, indices) in self.table_link_dependency_levels.iter().enumerate() {
            if indices.is_empty() {
                continue;
            }

            let mut handles = Vec::new();
            let handles_name = format!("level_{level}_handles");

            for index in indices {
                let table = &self.tables[*index];
                let name = &table.name;

                let (file, sheet) = match &table.schema {
                    TableSchema::Concrete { data, sheet, .. } => (data, sheet),
                    TableSchema::Abstract { .. } => continue,
                };

                let file = name.parent_namespace().join("/") + &format!("/{file}");

                handles.push(format!(
                    "{TAB}add::<{}::{}>({}, \"{}\", &mut {});",
                    name.parent_namespace().join("::"),
                    name.as_data_type(),
                    format!("data_dir.join(\"{}\")", file),
                    sheet,
                    handles_name,
                ));
            }

            let handles_code = handles.join("\n");
            let code = format!(
r#"    let mut {handles_name} = Vec::new();
{handles_code}

    join({handles_name}).await?;"#
            );
            level_handles.push(code);
        }

        Ok(format!(
r#"
use calamine::Reader;

pub async fn load_all(data_dir: &std::path::PathBuf) -> Result<(), {CRATE_PREFIX}::LoadError> {{
    type HandleType = tokio::task::JoinHandle<Result<(), {CRATE_PREFIX}::LoadError>>;

    fn add<T: {CRATE_PREFIX}::Loadable>(
        file: std::path::PathBuf,
        sheet: &str,
        handles: &mut Vec<HandleType>,
    ) {{
        let sheet = sheet.to_owned();

        handles.push(tokio::task::spawn(async move {{
            let mut workbook: calamine::Ods<_> = calamine::open_workbook(file)?;
            let sheet = workbook
                .with_header_row(calamine::HeaderRow::Row({HEADER_ROWS}))
                .worksheet_range(&sheet)?;
            T::load(&sheet.rows().collect::<Vec<_>>())?;

            Ok(())
        }}));
    }}

    async fn join(handles: Vec<HandleType>) -> Result<(), {CRATE_PREFIX}::LoadError> {{
        for handle in handles {{
            match handle.await {{
                Ok(result) => result?,
                _  => panic!("Data loading task has failed!"),
            }}
        }}

        Ok(())
    }}

{level_handles_code}

    Ok(())
}}
"#,
            level_handles_code = level_handles.join("\n"),
        ))
    }
}
