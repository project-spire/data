// This is a generated file. DO NOT MODIFY.
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum EquipmentKind {
    Weapon,
    Helmet,
    Armor,
}

impl EquipmentKind {
    pub fn parse(value: &calamine::Data) -> Result<Self, Error> {
        let enum_string = crate::parse_string(value)?;

        Ok(match enum_string.as_str() {
            "Weapon" => Self::Weapon,
            "Helmet" => Self::Helmet,
            "Armor" => Self::Armor,
            _ => return Err(Error::Parse(format!(
                "Invalid value \"{enum_string}\" for EquipmentKind"
            ))),
        })
    }

    pub fn try_from(value: &u8) -> Option<Self> {
        Some(match value {
            0 => Self::Weapon,
            1 => Self::Helmet,
            2 => Self::Armor,
            _ => return None,
        })
    }
}

impl Into<u8> for EquipmentKind {
    fn into(self) -> u8 {
        match self {
            Self::Weapon => 0,
            Self::Helmet => 1,
            Self::Armor => 2,
        }
    }
}
