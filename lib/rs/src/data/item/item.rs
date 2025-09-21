// This is a generated file. DO NOT MODIFY.
#![allow(static_mut_refs)]

use std::collections::HashMap;
use crate::{DataId, error::Error};

static mut ITEM_DATA: tokio::sync::OnceCell<ItemData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub enum Item {
    Equipment(&'static crate::item::Equipment),
    RandomBox(&'static crate::item::RandomBox),
}

#[derive(Debug)]
pub struct ItemData {
    data: HashMap<DataId, Item>,
}

impl Item {
    pub fn id(&self) -> &DataId {
        match self {
            Self::Equipment(x) => &x.id(),
            Self::RandomBox(x) => &x.id,
        }
    }
}

impl crate::Linkable for Item {
    fn get(id: &DataId) -> Option<&'static Self> {
        ItemData::get(id)
    }
}

impl ItemData {
    pub fn get(id: &DataId) -> Option<&'static Item> {
        let data = unsafe { &ITEM_DATA.get().unwrap().data };
        data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static Item)> {
        let data = unsafe { &ITEM_DATA.get().unwrap().data };
        data.iter()
    }
    
    pub(crate) fn init() {
        let data = HashMap::new();
        unsafe { ITEM_DATA.set(Self { data }).unwrap(); }
    }

    pub(crate) fn insert(id: &DataId, row: Item) -> Result<(), Error> {
        let data = unsafe { &mut ITEM_DATA.get_mut().unwrap().data };
        if data.contains_key(id) {
            return Err(Error::DuplicateId {
                type_name: std::any::type_name::<Item>(),
                id: *id,
            });
        }
        data.insert(*id, row);

        Ok(())
    }
}
