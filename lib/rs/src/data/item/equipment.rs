// This is a generated file. DO NOT MODIFY.
use std::collections::HashMap;
use tracing::info;
use crate::{DataId, error::Error};

static EQUIPMENT_DATA: tokio::sync::OnceCell<EquipmentData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub struct Equipment {
    pub id: DataId,
    pub name: String,
    pub kind: crate::item::EquipmentKind,
}

pub struct EquipmentData {
    data: HashMap<DataId, Equipment>,
}

impl Equipment {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), Error> {
        const FIELDS_COUNT: usize = 3;

        if row.len() != FIELDS_COUNT {
            return Err(Error::OutOfRange { expected: FIELDS_COUNT, actual: row.len() });
        }

        let id = crate::parse_id(&row[0])?;
        let name = crate::parse_string(&row[1])?;
        let kind = crate::item::EquipmentKind::parse(&row[2])?;

        Ok((id, Self {
            id,
            name,
            kind,
        }))
    }
}

impl crate::Linkable for Equipment {
    fn get(id: &DataId) -> Option<&'static Self> {
        EquipmentData::get(id)
    }
}

impl EquipmentData {
    pub fn get(id: &DataId) -> Option<&'static Equipment> {
        EQUIPMENT_DATA.get().unwrap().data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static Equipment)> {
        EQUIPMENT_DATA.get().unwrap().data.iter()
    }
}

impl crate::Loadable for EquipmentData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = HashMap::new();
        for row in rows {
            let (id, object) = Equipment::parse(row)?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicatedId {
                    type_name: std::any::type_name::<Equipment>(),
                    id,
                });
            }

            objects.insert(id, object);
        }

        if !EQUIPMENT_DATA.set(Self { data: objects }).is_ok() {
            return Err(Error::AlreadyLoaded {
                type_name: std::any::type_name::<Equipment>(),
            });
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }
}
