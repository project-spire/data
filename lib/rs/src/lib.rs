pub mod data;
pub use crate::data::*;
pub(crate) use parse::*;

mod parse;
mod error;

use std::fmt::Formatter;
use std::mem::MaybeUninit;
use std::ops::Deref;
use crate::error::{Error, LinkError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DataId(u32);

#[derive(Debug)]
pub struct Link<'a, T: Linkable> {
    id: DataId,
    target: MaybeUninit<&'a T>,
}

pub trait Linkable: Sized {
    fn get(id: &DataId) -> Option<&'static Self>;
}

pub(crate) trait Loadable: Sized {
    fn load(rows: &[&[calamine::Data]]) -> impl Future<Output = Result<(), Error>> + Send;
    fn init() -> Result<(), Error>;
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

impl<'a, T: Linkable> Deref for Link<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.target.assume_init_ref() }
    }
}

impl<'a, T: Linkable + 'static> Link<'a, T> {
    pub(crate) fn init(&mut self) -> Result<(), LinkError> {
        let target = match T::get(&self.id) {
            Some(target) => target,
            None => return Err(LinkError::Missing {
                type_name: std::any::type_name::<T>(),
                id: self.id,
            }),
        };

        self.target.write(target);
        Ok(())
    }
}
