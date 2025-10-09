// This is a generated file. DO NOT MODIFY.
use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "race")]
pub enum Race {
    None,
    Human,
    Orc,
}

impl Race {
    pub fn parse(value: &calamine::Data) -> Result<Self, ParseError> {
        let enum_string = crate::parse_string(value)?;

        Ok(match enum_string.as_str() {
            "None" => Self::None,
            "Human" => Self::Human,
            "Orc" => Self::Orc,
            _ => return Err(ParseError::InvalidValue {
                type_name: std::any::type_name::<Race>(),
                value: enum_string,
            }),
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

impl Into<protocol::Race> for Race {
    fn into(self) -> protocol::Race {
        type Target = protocol::Race;

        match self {
            Self::None => Target::None,
            Self::Human => Target::Human,
            Self::Orc => Target::Orc,
        }
    }
}

impl Into<Race> for protocol::Race {
    fn into(self) -> Race {
        type Target = Race;

        match self {
            Self::None => Target::None,
            Self::Human => Target::Human,
            Self::Orc => Target::Orc,
        }
    }
}

