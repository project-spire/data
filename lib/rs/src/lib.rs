pub mod data;
pub use crate::data::*;
pub(crate) use parse::*;

mod parse;
mod error;

use std::ops::Deref;
use std::fmt::Formatter;
use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DataId(u32);

#[derive(Debug)]
pub struct Link<'a, T> {
    id: DataId,
    target: &'a T,
}

pub trait Linkable: Sized {
    fn get(id: &DataId) -> Option<&'static Self>;
}

pub trait Loadable: Sized {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error>;
}

impl std::fmt::Display for DataId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataId({})", self.0)
    }
}

impl Deref for DataId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for DataId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = u32::from_str(s)?;
        Ok(DataId(id))
    }
}

impl<'a, T> Deref for Link<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.target
    }
}
