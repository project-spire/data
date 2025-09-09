mod collect;
mod generator;
mod error;

use heck::{ToUpperCamelCase, ToSnakeCase};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde::Deserialize;
use crate::error::Error;
use crate::generator::Generator;

const HEADER_ROWS: usize = 1;
const TAB: &str = "    ";
const CRATE_PREFIX: &str = "crate";
const GENERATED_FILE_WARNING: &str = r#"// This is a generated file. DO NOT MODIFY."#;

#[derive(Debug)]
pub struct Config {
    pub base_module_path: PathBuf,
    pub data_dir: PathBuf,
    pub gen_dir: PathBuf,
    pub dry_run: bool,
}

impl Config {
    pub fn generate(self) -> Result<(), Error> {
        let mut generator = Generator::new(self);
        generator.collect()?;
        generator.generate()?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Name {
    pub name: String,
    pub namespaces: Vec<String>,
}

impl Name {
    pub fn new(name: &str, namespaces: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            namespaces: namespaces.to_vec(),
        }
    }

    pub fn as_entity(&self) -> String {
        self.name.to_snake_case()
    }

    pub fn as_type(&self, with_namespace: bool) -> String {
        if with_namespace && !self.namespaces.is_empty() {
            format!(
                "{}::{}",
                self.namespaces.join("::"),
                self.name.to_upper_camel_case(),
            )
        } else {
            self.name.to_upper_camel_case()
        }
    }

    pub fn as_data_type(&self) -> String {
        format!("{}Data", self.as_type(false))
    }

    pub fn as_data_type_cell(&self) -> String {
        format!("{}_DATA", self.name.to_snake_case().to_uppercase())
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Target {
    Client,
    Server,
    All,
}

impl Target {
    fn is_target(&self) -> bool {
        self == &Target::Server || self == &Target::All
    }
}
