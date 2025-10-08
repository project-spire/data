// This is a generated file. DO NOT MODIFY.
#![allow(static_mut_refs)]

use std::collections::HashMap;
use std::mem::MaybeUninit;
use tracing::info;
use crate::{DataId, error::Error, error::ParseError};

static mut RANDOM_BOX_DATA: MaybeUninit<RandomBoxData> = MaybeUninit::uninit();

#[derive(Debug)]
pub struct RandomBox {
    pub id: DataId,
    pub name: String,
    pub items: (crate::Link<'static, crate::item::Item>, u16),
}

pub struct RandomBoxData {
    data: HashMap<DataId, RandomBox>,
}

impl RandomBox {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), ParseError> {
        const FIELDS_COUNT: usize = 3;

        if row.len() != FIELDS_COUNT {
            return Err(ParseError::InvalidColumnCount { expected: FIELDS_COUNT, actual: row.len() });
        }

        let id = crate::parse_id(&row[0])?;
        let name = crate::parse_string(&row[1])?;
        let items = crate::parse_tuple_2::<crate::Link<'static, crate::item::Item>, u16>(&row[2])?;

        Ok((id, Self {
            id,
            name,
            items,
        }))
    }
}

impl crate::Linkable for RandomBox {
    fn get(id: &DataId) -> Option<&'static Self> {
        RandomBoxData::get(id)
    }
}

impl RandomBoxData {
    pub fn get(id: &DataId) -> Option<&'static RandomBox> {
        unsafe { RANDOM_BOX_DATA.assume_init_ref() }.data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static RandomBox)> {
        unsafe { RANDOM_BOX_DATA.assume_init_ref() }.data.iter()
    }
}

impl crate::Loadable for RandomBoxData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = HashMap::new();
        let mut index = 2;
        for row in rows {
            let (id, object) = RandomBox::parse(row)
                .map_err(|e| Error::Parse {
                    workbook: "random_box.ods",
                    sheet: "RandomBox",
                    row: index + 1,
                    error: e,
                })?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicateId {
                    type_name: std::any::type_name::<RandomBox>(),
                    id,
                });
            }

            objects.insert(id, object);
            
            index += 1;
        }

        let data = Self { data: objects };
        unsafe { RANDOM_BOX_DATA.write(data); }

        for (id, row) in unsafe { RANDOM_BOX_DATA.assume_init_ref() }.data.iter() {
            crate::item::ItemData::insert(&id, crate::item::Item::RandomBox(row))?;
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }

    fn init() -> Result<(), Error> {
        Ok(())
    }
}
