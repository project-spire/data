// This is a generated file. DO NOT MODIFY.
use tracing::info;
use crate::{DataId, error::Error};

pub static RANDOM_BOX_DATA: tokio::sync::OnceCell<RandomBoxData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub struct RandomBox {
    pub id: DataId,
    pub name: String,
    pub items: (crate::Link<'static, crate::item::Item>, u16),
}

impl RandomBox {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), Error> {
        const FIELDS_COUNT: usize = 3;

        if row.len() != FIELDS_COUNT {
            return Err(Error::OutOfRange { expected: FIELDS_COUNT, actual: row.len() });
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
    fn get(id: DataId) -> Option<&'static Self> {
        RandomBoxData::get(id)
    }
}

pub struct RandomBoxData {
    data: std::collections::HashMap<DataId, RandomBox>,
}

impl RandomBoxData {
    pub fn get(id: DataId) -> Option<&'static RandomBox> {
        RANDOM_BOX_DATA
            .get()
            .expect("RANDOM_BOX_DATA is not initialized yet")
            .data
            .get(&id)
    }
}

impl crate::Loadable for RandomBoxData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = std::collections::HashMap::new();
        for row in rows {
            let (id, object) = RandomBox::parse(row)?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicatedId {
                    type_name: std::any::type_name::<RandomBox>(),
                    id,
                });
            }

            objects.insert(id, object);
        }

        if !RANDOM_BOX_DATA.set(Self { data: objects }).is_ok() {
            return Err(Error::AlreadyLoaded {
                type_name: std::any::type_name::<RandomBox>(),
            });
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }
}
