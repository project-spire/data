// This is a generated file. DO NOT MODIFY.
pub mod item;
pub mod character;



use calamine::Reader;
use crate::error::Error;

pub async fn load_all(data_dir: &std::path::PathBuf) -> Result<(), Error> {
    type HandleType = tokio::task::JoinHandle<Result<(), Error>>;

    fn add<T: crate::Loadable>(
        file: std::path::PathBuf,
        sheet: &str,
        handles: &mut Vec<HandleType>,
    ) {
        let sheet = sheet.to_owned();

        handles.push(tokio::task::spawn(async move {
            let mut workbook: calamine::Ods<_> = calamine::open_workbook(file)?;
            let sheet = workbook
                .with_header_row(calamine::HeaderRow::Row(1))
                .worksheet_range(&sheet)?;
            T::load(&sheet.rows().collect::<Vec<_>>())?;

            Ok(())
        }));
    }

    async fn join(handles: Vec<HandleType>) -> Result<(), Error> {
        for handle in handles {
            match handle.await {
                Ok(result) => result?,
                _  => panic!("Data loading task has failed!"),
            }
        }

        Ok(())
    }

    item::EquipmentData::init();
    item::ItemData::init();

    let mut level_0_handles = Vec::new();
    add::<item::RandomBoxData>(data_dir.join("item/random_box.ods"), "RandomBox", &mut level_0_handles);
    add::<character::RaceStatData>(data_dir.join("character/race_stat.ods"), "RaceStat", &mut level_0_handles);
    add::<item::WeaponData>(data_dir.join("item/weapon.ods"), "Weapon", &mut level_0_handles);

    join(level_0_handles).await?;

    Ok(())
}
