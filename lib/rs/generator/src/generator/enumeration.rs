use std::fs;
use crate::*;
use crate::generator::*;

#[derive(Debug)]
pub struct EnumerationEntry {
    pub name: Name,
    pub schema: EnumerationSchema
}

#[derive(Debug, Deserialize)]
pub struct EnumerationSchema {
    name: String,
    base: EnumerationBase,
    enums: Vec<String>,
    target: Target,
    #[serde(default)]
    protocol: bool,
    attributes: Vec<EnumerationAttribute>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnumerationBase {
    Uint8,
    Uint16,
    Uint32,
}

#[derive(Debug, Deserialize)]
pub struct EnumerationAttribute {
    target: Target,
    attribute: String,
}

impl Generator {
    pub fn generate_enumeration(&self, enumeration: &EnumerationEntry) -> Result<(), Error> {
        let enumeration_file = self
            .full_gen_dir(&enumeration.name.parent_namespace())
            .join(format!("{}.rs", enumeration.name.as_entity()));
        self.log(&format!("Generating enumeration `{}`", enumeration_file.display()));

        let mut code = self.generate_code(&enumeration.schema)?;
        if enumeration.schema.protocol {
            code += &self.generate_protocol(&enumeration.name, &enumeration.schema)?;
        }

        fs::write(enumeration_file, code)?;
        Ok(())
    }

    fn generate_code(&self, schema: &EnumerationSchema) -> Result<String, Error> {
        let mut enums = Vec::new();
        let mut enum_parses = Vec::new();
        let mut enum_intos = Vec::new();
        let mut enum_froms = Vec::new();
        let mut attributes = Vec::new();

        let mut index: u32 = 0;
        for e in &schema.enums {
            enums.push(format!("{TAB}{e},"));
            enum_parses.push(format!("{TAB}{TAB}{TAB}\"{e}\" => Self::{e},"));
            enum_intos.push(format!("{TAB}{TAB}{TAB}Self::{e} => {index},"));
            enum_froms.push(format!("{TAB}{TAB}{TAB}{index} => Self::{e},"));

            index += 1;
        }

        for attribute in &schema.attributes {
            if !attribute.target.is_target() {
                continue;
            }

            attributes.push(attribute.attribute.clone());
        }

        let enums_code = enums.join("\n");
        let attributes_code = attributes.join("\n");
        let enum_parses_code = enum_parses.join("\n");
        let enum_intos_code = enum_intos.join("\n");
        let enum_froms_code = enum_froms.join("\n");

        Ok(format!(
            r#"{GENERATED_FILE_WARNING}
use crate::error::ParseError;
use crate::parse::*;

{attributes_code}
pub enum {enum_type_name} {{
{enums_code}
}}

impl {enum_type_name} {{
    pub fn parse(value: &calamine::Data) -> Result<Self, ParseError> {{
        let enum_string = String::parse(value)?;

        Ok(match enum_string.as_str() {{
{enum_parses_code}
            _ => return Err(ParseError::InvalidValue {{
                type_name: std::any::type_name::<{enum_type_name}>(),
                value: enum_string,
            }}),
        }})
    }}

    pub fn try_from(value: &{base_type_name}) -> Option<Self> {{
        Some(match value {{
{enum_froms_code}
            _ => return None,
        }})
    }}
}}

impl Into<{base_type_name}> for {enum_type_name} {{
    fn into(self) -> {base_type_name} {{
        match self {{
{enum_intos_code}
        }}
    }}
}}
"#,
            enum_type_name = schema.name,
            base_type_name = schema.base.to_rust_type(),
        ))
    }

    fn generate_protocol(&self, name: &Name, schema: &EnumerationSchema) -> Result<String, Error> {
        let protocol_file = self.config.protocol_gen_dir
            .join(format!("{}.gen.proto", name.name));
        self.log(&format!("Generating enumeration protocol `{}`", protocol_file.display()));

        let mut enums = Vec::new();
        let mut enum_matches = Vec::new();

        let mut index: u32 = 0;
        for e in &schema.enums {
            enums.push(format!("{TAB}{e} = {index};"));
            enum_matches.push(format!("{TAB}{TAB}{TAB}Self::{e} => Target::{e},"));

            index += 1;
        }

        let code = format!(
r#"{GENERATED_FILE_WARNING}
syntax = "proto3";

package spire.protocol;

enum {enum_type_name} {{
{enums_code}
}}
"#,
            enum_type_name = name.as_type(false),
            enums_code = enums.join("\n"),
        );
        fs::write(protocol_file, code)?;

        Ok(format!(
r#"
impl Into<protocol::{enum_type_name}> for {enum_type_name} {{
    fn into(self) -> protocol::{enum_type_name} {{
        type Target = protocol::Race;

        match self {{
{enum_matches_code}
        }}
    }}
}}

impl Into<{enum_type_name}> for protocol::{enum_type_name} {{
    fn into(self) -> {enum_type_name} {{
        type Target = {enum_type_name};

        match self {{
{enum_matches_code}
        }}
    }}
}}

"#,
            enum_type_name = name.as_type(false),
            enum_matches_code = enum_matches.join("\n"),
        ))
    }
}

impl EnumerationBase {
    fn to_rust_type(&self) -> &str {
        match self {
            EnumerationBase::Uint8 => "u8",
            EnumerationBase::Uint16 => "u16",
            EnumerationBase::Uint32 => "u32",
        }
    }
}
