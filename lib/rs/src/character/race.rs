// This is a generated file. DO NOT MODIFY.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Race {
    None,
    Human,
    Orc,
}

impl Race {
    pub fn parse(value: &calamine::Data) -> Result<Self, crate::LoadError> {
        let enum_string = crate::parse_string(value)?;

        Ok(match enum_string.as_str() {
            "None" => Self::None,
            "Human" => Self::Human,
            "Orc" => Self::Orc,
            _ => return Err(crate::LoadError::Parse(format!(
                "Invalid value \"{enum_string}\" for Race"
            ))),
        })
    }

    pub fn try_from(value: &u16) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Human,
            2 => Self::Orc,
            _ => return None,
        })
    }
}

impl Into<u16> for Race {
    fn into(self) -> u16 {
        match self {
            Self::None => 0,
            Self::Human => 1,
            Self::Orc => 2,
        }
    }
}
