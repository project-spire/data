pub mod data;
pub use crate::data::*;

mod error;
mod parse;

use crate::error::{Error, LinkError};
use std::fmt::Formatter;
use std::mem::MaybeUninit;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DataId(u32);

#[derive(Debug)]
pub struct Link<T: Linkable + 'static> {
    id: DataId,
    target: MaybeUninit<&'static T>,
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

impl<T: Linkable> Deref for Link<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.target.assume_init_ref() }
    }
}

impl<T: Linkable + 'static> Link<T> {
    pub(crate) fn init(&mut self) -> Result<(), LinkError> {
        let target = match T::get(&self.id) {
            Some(target) => target,
            None => {
                return Err(LinkError::Missing {
                    type_name: std::any::type_name::<T>(),
                    id: self.id,
                });
            }
        };

        self.target.write(target);
        Ok(())
    }
}
