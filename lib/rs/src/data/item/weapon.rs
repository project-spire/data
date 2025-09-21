// This is a generated file. DO NOT MODIFY.
use std::collections::HashMap;
use tracing::info;
use crate::{DataId, error::Error};

static WEAPON_DATA: tokio::sync::OnceCell<WeaponData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub struct Weapon {
    pub id: DataId,
    pub name: String,
    pub weight: u16,
    pub damage: u32,
}

pub struct WeaponData {
    data: HashMap<DataId, Weapon>,
}

impl Weapon {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), Error> {
        const FIELDS_COUNT: usize = 4;

        if row.len() != FIELDS_COUNT {
            return Err(Error::OutOfRange { expected: FIELDS_COUNT, actual: row.len() });
        }

        let id = crate::parse_id(&row[0])?;
        let name = crate::parse_string(&row[1])?;
        let weight = crate::parse_u16(&row[2])?;
        let damage = crate::parse_u32(&row[3])?;

        Ok((id, Self {
            id,
            name,
            weight,
            damage,
        }))
    }
}

impl crate::Linkable for Weapon {
    fn get(id: &DataId) -> Option<&'static Self> {
        WeaponData::get(id)
    }
}

impl WeaponData {
    pub fn get(id: &DataId) -> Option<&'static Weapon> {
        WEAPON_DATA.get().unwrap().data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static Weapon)> {
        WEAPON_DATA.get().unwrap().data.iter()
    }
}

impl crate::Loadable for WeaponData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = HashMap::new();
        for row in rows {
            let (id, object) = Weapon::parse(row)?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicateId {
                    type_name: std::any::type_name::<Weapon>(),
                    id,
                });
            }

            objects.insert(id, object);
        }

        if !WEAPON_DATA.set(Self { data: objects }).is_ok() {
            return Err(Error::AlreadyLoaded {
                type_name: std::any::type_name::<Weapon>(),
            });
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }
}
