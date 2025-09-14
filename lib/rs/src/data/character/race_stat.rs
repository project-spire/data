// This is a generated file. DO NOT MODIFY.
use std::collections::HashMap;
use tracing::info;
use crate::{DataId, error::Error};

static RACE_STAT_DATA: tokio::sync::OnceCell<RaceStatData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub struct RaceStat {
    pub id: DataId,
    pub race: crate::character::Race,
    pub speed: f32,
}

impl RaceStat {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), Error> {
        const FIELDS_COUNT: usize = 3;

        if row.len() != FIELDS_COUNT {
            return Err(Error::OutOfRange { expected: FIELDS_COUNT, actual: row.len() });
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
    fn get(id: DataId) -> Option<&'static Self> {
        RaceStatData::get(id)
    }
}

pub struct RaceStatData {
    data: HashMap<DataId, RaceStat>,
}

impl RaceStatData {
    pub fn get(id: DataId) -> Option<&'static RaceStat> {
        RACE_STAT_DATA.get().unwrap().data.get(&id)
    }

    pub fn iter() -> impl Iterator<Item = (&'static DataId, &'static RaceStat)> {
        RACE_STAT_DATA.get().unwrap().data.iter()
    }
}

impl crate::Loadable for RaceStatData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), Error> {
        let mut objects = HashMap::new();
        for row in rows {
            let (id, object) = RaceStat::parse(row)?;

            if objects.contains_key(&id) {
                return Err(Error::DuplicatedId {
                    type_name: std::any::type_name::<RaceStat>(),
                    id,
                });
            }

            objects.insert(id, object);
        }

        if !RACE_STAT_DATA.set(Self { data: objects }).is_ok() {
            return Err(Error::AlreadyLoaded {
                type_name: std::any::type_name::<RaceStat>(),
            });
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }
}
