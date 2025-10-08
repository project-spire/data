// This is a generated file. DO NOT MODIFY.
pub mod item;
pub mod character;



use calamine::Reader;
use crate::error::Error;

pub async fn load_all(data_dir: &std::path::PathBuf) -> Result<(), Error> {
    type TaskType = tokio::task::JoinHandle<Result<(), Error>>;

    fn load<T: crate::Loadable>(
        file: std::path::PathBuf,
        sheet: &str,
        tasks: &mut Vec<TaskType>,
    ) {
        let sheet = sheet.to_owned();

        tasks.push(tokio::task::spawn(async move {
            let mut workbook: calamine::Ods<_> = calamine::open_workbook(file)?;
            let sheet = workbook
                .with_header_row(calamine::HeaderRow::Row(2))
                .worksheet_range(&sheet)?;
            T::load(&sheet.rows().collect::<Vec<_>>())?;

            Ok(())
        }));
    }

    fn init<T: crate::Loadable>(
        tasks: &mut Vec<TaskType>,
    ) {
        tasks.push(tokio::task::spawn(async move {
            T::init()?;
            Ok(())
        }));
    }

    // Initialize abstract tables
    item::EquipmentData::init();
    item::ItemData::init();

    // Load concrete tables asynchronously
    let mut load_tasks = Vec::new();
    load::<item::RandomBoxData>(data_dir.join("item/random_box.ods"), "RandomBox", &mut load_tasks);
    load::<item::WeaponData>(data_dir.join("item/weapon.ods"), "Weapon", &mut load_tasks);
    load::<character::RaceStatData>(data_dir.join("character/race_stat.ods"), "RaceStat", &mut load_tasks);

    for task in load_tasks {
        match task.await {
            Ok(result) => result?,
            _  => panic!("Data loading task has failed!"),
        }
    }

    // Initialize concrete tables asynchronously
    let mut init_tasks = Vec::new();
    init::<item::RandomBoxData>(&mut init_tasks);
    init::<item::WeaponData>(&mut init_tasks);
    init::<character::RaceStatData>(&mut init_tasks);

    for task in init_tasks {
        match task.await {
            Ok(result) => result?,
            _  => panic!("Data initializing task has failed!"),
        }
    }

    Ok(())
}
