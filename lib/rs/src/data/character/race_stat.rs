// This is a generated file. DO NOT MODIFY.
#![allow(static_mut_refs)]

use std::collections::HashMap;
use std::mem::MaybeUninit;
use tracing::info;
use crate::{DataId, error::Error, error::ParseError};

static mut RACE_STAT_DATA: MaybeUninit<RaceStatData> = MaybeUninit::uninit();

#[derive(Debug)]
pub struct RaceStat {
    pub id: DataId,
    pub race: crate::character::Race,
    pub speed: f32,
}

pub struct RaceStatData {
    data: HashMap<DataId, RaceStat>,
}

impl RaceStat {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), ParseError> {
        const FIELDS_COUNT: usize = 3;

        if row.len() != FIELDS_COUNT {
            return Err(ParseError::InvalidColumnCount { expected: FIELDS_COUNT, actual: row.len() });
        }

        let id = crate::parse_id(&row[0])?;
        let race = crate::character::Race::parse(&row[1])?;
        let speed = crate::parse_f32(&row[2])?;

        Ok((id, Self {
            id,
            race,
            speed,
        }))
    }
}

impl crate::Linkable for RaceStat {
    fn get(id: &DataId) -> Option<&'static Self> {
        RaceStatData::get(id)
    }
}

impl RaceStatData {
    pub fn get(id: &DataId) -> Option<&'static RaceStat> {
        unsafe { RACE_STAT_DATA.assume_init_ref() }.data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static RaceStat)> {
        unsafe { RACE_STAT_DATA.assume_init_ref() }.data.iter()
    }
}

impl crate::Loadable for RaceStatData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = HashMap::new();
        let mut index = 2;
        for row in rows {
            let (id, object) = RaceStat::parse(row)
                .map_err(|e| Error::Parse {
                    workbook: "race_stat.ods",
                    sheet: "RaceStat",
                    row: index + 1,
                    error: e,
                })?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicateId {
                    type_name: std::any::type_name::<RaceStat>(),
                    id,
                });
            }

            objects.insert(id, object);
            
            index += 1;
        }

        let data = Self { data: objects };
        unsafe { RACE_STAT_DATA.write(data); }

        for (id, row) in unsafe { RACE_STAT_DATA.assume_init_ref() }.data.iter() {
            
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }

    fn init() -> Result<(), Error> {
        Ok(())
    }
}
