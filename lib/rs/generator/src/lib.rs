mod collect;
mod generator;
mod error;
mod name;

use std::path::PathBuf;
use serde::Deserialize;
use crate::name::Name;
use crate::error::Error;
use crate::generator::Generator;

const HEADER_ROWS: usize = 1;
const TAB: &str = "    ";
const CRATE_PREFIX: &str = "crate";
const GENERATED_FILE_WARNING: &str = r#"// This is a generated file. DO NOT MODIFY."#;

#[derive(Debug)]
pub struct Config {
    pub schema_dir: PathBuf,
    pub src_dir: PathBuf,
    pub gen_dir: PathBuf,
    pub protocol_gen_dir: PathBuf,

    pub verbose: bool,
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
