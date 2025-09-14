// This is a generated file. DO NOT MODIFY.
#![allow(static_mut_refs)]

use std::collections::HashMap;
use crate::{DataId, error::Error};

static mut EQUIPMENT_DATA: tokio::sync::OnceCell<EquipmentData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub enum Equipment {
    Weapon(&'static crate::item::Weapon),
}

#[derive(Debug)]
pub struct EquipmentData {
    data: HashMap<DataId, Equipment>,
}

impl Equipment {
    pub fn id(&self) -> &DataId {
        match self {
            Self::Weapon(x) => &x.id,
        }
    }
}

impl crate::Linkable for Equipment {
    fn get(id: &DataId) -> Option<&'static Self> {
        EquipmentData::get(id)
    }
}

impl EquipmentData {
    pub fn get(id: &DataId) -> Option<&'static Equipment> {
        let data = unsafe { &EQUIPMENT_DATA.get().unwrap().data };
        data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static Equipment)> {
        let data = unsafe { &EQUIPMENT_DATA.get().unwrap().data };
        data.iter()
    }

    pub(crate) fn insert(id: &DataId, row: Equipment) -> Result<(), Error> {
        let data = unsafe { &mut EQUIPMENT_DATA.get_mut().unwrap().data };
        if data.contains_key(id) {
            return Err(Error::DuplicateId {
                type_name: std::any::type_name::<Equipment>(),
                id: *id,
            });
        }
        data.insert(*id, row);

        crate::item::ItemData::insert(id, crate::item::Item::Equipment(&data[id]))?;

        Ok(())
    }
}

impl crate::Loadable for EquipmentData {
    fn load(_: &[&[calamine::Data]]) -> Result<(), Error> {
        let data = HashMap::new();
        unsafe { EQUIPMENT_DATA.set(Self { data }).unwrap(); }

        Ok(())
    }
}
