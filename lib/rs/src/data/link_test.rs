// This is a generated file. DO NOT MODIFY.
#![allow(static_mut_refs)]

use std::collections::HashMap;
use std::mem::MaybeUninit;
use tracing::info;
use crate::{DataId, error::*};

static mut LINK_TEST_DATA: MaybeUninit<LinkTestData> = MaybeUninit::uninit();

#[derive(Debug)]
pub struct LinkTest {
    pub id: DataId,
    pub item_link: crate::Link<'static, crate::item::Item>,
    pub optional_item_link: crate::Link<'static, crate::item::Item>,
    pub multi_item_link: crate::Link<'static, crate::item::Item>,
}

pub struct LinkTestData {
    data: HashMap<DataId, LinkTest>,
}

impl LinkTest {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), (&'static str, ParseError)> {
        const FIELDS_COUNT: usize = 4;

        if row.len() != FIELDS_COUNT {
            return Err(("", ParseError::InvalidColumnCount { expected: FIELDS_COUNT, actual: row.len() }));
        }

        let id = crate::parse_id(&row[0]).map_err(|e| ("id", e))?;
        let item_link = crate::parse_link::<crate::item::Item>(&row[1]).map_err(|e| ("item_link", e))?;
        let optional_item_link = crate::parse_link::<crate::item::Item>(&row[2]).map_err(|e| ("optional_item_link", e))?;
        let multi_item_link = crate::parse_link::<crate::item::Item>(&row[3]).map_err(|e| ("multi_item_link", e))?;

        Ok((id, Self {
            id,
            item_link,
            optional_item_link,
            multi_item_link,
        }))
    }
}

impl crate::Linkable for LinkTest {
    fn get(id: &DataId) -> Option<&'static Self> {
        LinkTestData::get(id)
    }
}

impl LinkTestData {
    pub fn get(id: &DataId) -> Option<&'static LinkTest> {
        unsafe { LINK_TEST_DATA.assume_init_ref() }.data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static LinkTest)> {
        unsafe { LINK_TEST_DATA.assume_init_ref() }.data.iter()
    }
}

impl crate::Loadable for LinkTestData {
    async fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = HashMap::new();
        let mut index = 2;
        for row in rows {
            let (id, object) = LinkTest::parse(row)
                .map_err(|(column, error)| Error::Parse {
                    workbook: "link_test.ods",
                    sheet: "LinkTest",
                    row: index + 1,
                    column,
                    error,
                })?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicateId {
                    type_name: std::any::type_name::<LinkTest>(),
                    id,
                    a: format!("{:?}", objects[&id]),
                    b: format!("{:?}", object),
                });
            }

            objects.insert(id, object);
            
            index += 1;
        }

        let data = Self { data: objects };
        unsafe { LINK_TEST_DATA.write(data); }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }

    fn init() -> Result<(), Error> {
        fn link(data: &mut HashMap<DataId, LinkTest>) -> Result<(), (DataId, LinkError)> {
            for (id, row) in data {
                row.item_link.init().map_err(|e| (*id, e))?;
                row.optional_item_link.init().map_err(|e| (*id, e))?;
                row.multi_item_link.init().map_err(|e| (*id, e))?;
            }

            Ok(())
        }

        link(&mut unsafe { LINK_TEST_DATA.assume_init_mut() }.data)
            .map_err(|(id, error)| Error::Link {
                workbook: "link_test.ods",
                sheet: "LinkTest",
                id,
                error,
            })?;


        Ok(())
    }
}
