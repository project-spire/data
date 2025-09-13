use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;
use serde::Deserialize;
use crate::*;
use crate::generator::constant::*;
use crate::generator::enumeration::*;
use crate::generator::module::*;
use crate::generator::table::*;

impl Generator {
    pub fn collect(&mut self) -> Result<(), Error> {
        println!("Collecting...");

        let base_module = ModuleEntry::new(Name::new("", Vec::new()));
        let mut next_modules = vec![base_module];

        while let Some(mut module) = next_modules.pop() {
            let child_modules = self.collect_entities(&mut module)?;
            next_modules.extend(child_modules);

            self.modules.push(module);
        }

        self.build_table_hierarchies()?;
        self.build_table_link_dependency_levels()?;

        Ok(())
    }

    fn collect_entities(
        &mut self,
        module: &mut ModuleEntry,
    ) -> Result<Vec<ModuleEntry>, Error> {
        let module_dir = module.name.as_full_path(&self.config.data_dir);

        // Collect entities
        let entity_files: Vec<PathBuf> = glob(module_dir.join("*.json").to_str().unwrap())
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        for entity_file in entity_files {
            let file_name = entity_file.file_name().unwrap().to_str().unwrap();
            let components = file_name.split('.').collect::<Vec<_>>();

            if components.len() != 3 {
                return Err(Error::InvalidFileName(file_name.to_owned()));
            }

            let entity_name = module.name.get_child(components[0]);
            match components[1] {
                "table" => {
                    self.collect_table(module, &entity_file, entity_name)?;
                },
                "enum" => {
                    self.collect_enumeration(module, &entity_file, entity_name)?;
                },
                "const" => {
                    self.collect_constant(module, &entity_file, entity_name)?;
                },
                _ => {
                    return Err(Error::InvalidFileName(file_name.to_owned()));
                }
            }

            println!("cargo:rerun-if-changed={}", entity_file.display());
        }

        // Collect child modules
        let mut child_modules = Vec::new();

        for entry in fs::read_dir(&module_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let module_name = module.name.get_child(dir_name);

            child_modules.push(ModuleEntry::new(module_name));
        }

        Ok(child_modules)
    }

    fn collect_table(
        &mut self,
        module: &mut ModuleEntry,
        file: &Path,
        name: Name,
    ) -> Result<(), Error> {
        let type_name = name.as_type(true);
        self.register_type(&type_name)?;

        let schema: TableSchema = serde_json::from_str(&fs::read_to_string(file)?)?;

        self.tables.push(TableEntry { name, schema });
        module.entries.push(EntityEntry::TableIndex(
            self.tables.len() - 1
        ));

        self.table_indices.insert(type_name, self.tables.len() - 1);

        Ok(())
    }

    fn collect_enumeration(
        &mut self,
        module: &mut ModuleEntry,
        file: &Path,
        name: Name,
    ) -> Result<(), Error> {
        self.register_type(&name.as_type(true))?;

        let schema: EnumerationSchema = serde_json::from_str(&fs::read_to_string(file)?)?;

        self.enumerations.push(EnumerationEntry {
            name,
            schema,
        });
        module.entries.push(EntityEntry::EnumerationIndex(
            self.enumerations.len() - 1
        ));

        Ok(())
    }

    fn collect_constant(
        &mut self,
        module: &mut ModuleEntry,
        file: &Path,
        name: Name,
    ) -> Result<(), Error> {
        self.register_type(&name.as_type(true))?;

        let schema: ConstantSchema = serde_json::from_str(&fs::read_to_string(file)?)?;

        self.constants.push(ConstantEntry {
            name,
            schema,
        });
        module.entries.push(EntityEntry::ConstantIndex(
            self.enumerations.len() - 1
        ));

        Ok(())
    }

    fn register_type(&mut self, type_name: &str) -> Result<(), Error> {
        if self.names.contains(type_name) {
            return Err(Error::NamespaceCollision(type_name.to_owned()));
        }

        self.names.insert(type_name.to_owned());
        Ok(())
    }

    fn build_table_hierarchies(&mut self) -> Result<(), Error> {
        for (index, _) in self.tables.iter().enumerate() {
            self.table_hierarchies.insert(index, Vec::new());
        }

        for (index, table) in self.tables.iter().enumerate() {
            let extend = match &table.schema {
                TableSchema::Concrete(schema) => &schema.extend,
                TableSchema::Abstract(schema) => &schema.extend,
            };
            let extend = if let Some(e) = extend {
                e
            } else {
                continue;
            };
            
            self.table_hierarchies.get_mut(&index).unwrap()
                .push(*self.table_indices.get(extend).unwrap());
        }
        
        Ok(())
    }

    fn build_table_link_dependency_levels(&mut self) -> Result<(), Error> {
        // 1. Build graph
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for (index, table) in self.tables.iter().enumerate() {
            graph.insert(index, Vec::new());

            let fields: &[Field] = match &table.schema {
                TableSchema::Concrete(schema) => &schema.fields,
                TableSchema::Abstract(schema) => &schema.fields,
            };

            for field in fields {
                let link_type = match &field.kind {
                    FieldKind::Link { link_type } => link_type,
                    _ => continue,
                };

                let target_index = match self.table_indices.get(link_type) {
                    Some(index) => *index,
                    None => {
                        return Err(Error::UnknownType(link_type.clone()));
                    },
                };

                graph.get_mut(&index).unwrap().push(target_index);
            }
        }

        // 2. Build in-degrees
        let mut in_degrees: HashMap<usize, usize> = graph
            .iter()
            .map(|(node, _)| (*node, 0))
            .collect();

        for nodes in graph.values() {
            for node in nodes {
                *in_degrees.get_mut(node).unwrap() += 1;
            }
        }

        // 3. Sort topologically (Khan's Algorithm)
        let mut queue: VecDeque<usize> = in_degrees
            .iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(node, _)| *node)
            .collect();
        let mut sorted_count = 0;

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::with_capacity(level_size);

            for _ in 0..level_size {
                let u = queue.pop_front().unwrap();
                for v in graph.get(&u).unwrap() {
                    let degree = in_degrees.get_mut(v).unwrap();
                    *degree -= 1;

                    if *degree == 0 {
                        queue.push_back(*v);
                    }
                }

                current_level.push(u);
                sorted_count += 1;
            }
            self.table_link_dependency_levels.push(current_level);
        }

        // Cycle detected
        if sorted_count != graph.len() {
            return Err(Error::CircularDependency)
        }

        Ok(())
    }
}
