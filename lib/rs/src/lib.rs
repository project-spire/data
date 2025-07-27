use std::ops::Deref;

pub mod generator;

pub type DataId = u32;

#[derive(Debug)]
pub struct Link<'a, T> {
    id: DataId,
    target: &'a T,
}

impl<T> Link<'_, T> {
    pub fn id(&self) -> DataId {
        self.id
    }
}

impl<T> Deref for Link<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.target
    }
}
