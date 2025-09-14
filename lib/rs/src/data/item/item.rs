// This is a generated file. DO NOT MODIFY.
use std::collections::HashMap;
use tracing::info;
use crate::{DataId, error::Error};

static ITEM_DATA: tokio::sync::OnceCell<ItemData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub enum Item {
    Equipment(&'static crate::item::Equipment),
    RandomBox(&'static crate::item::RandomBox),
}

pub struct ItemData {
    data: HashMap<DataId, Item>,
}

impl Item {
    pub fn id(&self) -> &DataId {
        match self {
            Self::Equipment(x) => &x.id,
            Self::RandomBox(x) => &x.id,
        }
    }
}

impl crate::Loadable for ItemData {
    fn load(_: &[&[calamine::Data]]) -> Result<(), Error> {
        fn check(data: &HashMap<DataId, Item>, id: &DataId) -> Result<(), Error> {
            if data.contains_key(id) {
                return Err(Error::DuplicatedId {
                    type_name: std::any::type_name::<Item>(),
                    id: *id,
                });
            }
            Ok(())
        };

        let mut data = HashMap::new();

        for (id, row) in crate::item::EquipmentData::iter() {
            check(&data, id)?;
            data.insert(*id, Item::Equipment(row));
        }
        for (id, row) in crate::item::RandomBoxData::iter() {
            check(&data, id)?;
            data.insert(*id, Item::RandomBox(row));
        }

        if !ITEM_DATA.set(Self { data }).is_ok() {
            return Err(Error::AlreadyLoaded {
                type_name: std::any::type_name::<Item>(),
            });
        }
        Ok(())
    }
}
