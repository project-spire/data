pub(crate) mod constant;
pub(crate) mod enumeration;
pub(crate) mod module;
pub(crate) mod table;

use std::fs;
use crate::*;
use self::constant::ConstantEntry;
use self::enumeration::EnumerationEntry;
use self::module::ModuleEntry;
use self::table::TableEntry;

#[derive(Debug)]
pub struct Generator {
    pub config: Config,

    pub base_module_index: Option<usize>,
    pub modules: Vec<ModuleEntry>,
    pub tables: Vec<TableEntry>,
    pub enumerations: Vec<EnumerationEntry>,
    pub constants: Vec<ConstantEntry>,

    pub type_names: HashSet<String>,
    pub table_indices: HashMap<String, usize>,
    pub dependency_levels: Vec<Vec<usize>>,
}

impl Generator {
    pub fn new(config: Config) -> Self {
        Self {
            config,

            base_module_index: None,
            modules: Vec::new(),
            tables: Vec::new(),
            enumerations: Vec::new(),
            constants: Vec::new(),

            type_names: HashSet::new(),
            table_indices: HashMap::new(),
            dependency_levels: Vec::new(),
        }
    }

    pub fn full_data_path(&self, namespaces: &[String], file_name: &str) -> PathBuf {
        self.config.data_dir.join(namespaces.join(".")).join(file_name)
    }

    pub fn full_gen_dir(&self, namespaces: &[String]) -> PathBuf {
        self.config.gen_dir.join(namespaces.join("/"))
    }

    pub fn full_gen_path(&self, namespaces: &[String], file_name: &str) -> PathBuf {
        self.full_gen_dir(namespaces).join(file_name)
    }

    pub fn generate(&self) -> Result<(), Error> {
        println!("Generating...");

        fs::create_dir_all(&self.config.gen_dir)?;

        for module in &self.modules {
            self.generate_module(module)?
        }

        for table in &self.tables {
            self.generate_table(&table)?
        }

        for enumeration in &self.enumerations {
            self.generate_enumeration(&enumeration)?
        }

        for constant in &self.constants {
            self.generate_constant(&constant)?
        }

        Ok(())
    }
}
