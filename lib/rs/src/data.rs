// This is a generated file. DO NOT MODIFY.
pub mod link_test;
pub mod item;
pub mod character;

pub use link_test::*;

use calamine::Reader;
use crate::error::Error;

pub async fn load_all(data_dir: &std::path::PathBuf) -> Result<(), Error> {
    init_abstract_tables();
    load_concrete_tables(data_dir).await?;
    init_concrete_tables().await?;

    Ok(())
}

fn init_abstract_tables() {
    item::EquipmentData::init();
    item::ItemData::init();
}

async fn load_concrete_tables(data_dir: &std::path::PathBuf) -> Result<(), Error> {
    fn load<T: crate::Loadable>(
        file: std::path::PathBuf,
        sheet: &str,
        tasks: &mut Vec<tokio::task::JoinHandle<Result<(), Error>>>,
    ) {
        let sheet = sheet.to_owned();

        tasks.push(tokio::task::spawn(async move {
            let mut workbook: calamine::Ods<_> = calamine::open_workbook(&file)
                .map_err(|error| Error::Workbook {
                    workbook: file.display().to_string(),
                    error,
                })?;
            let sheet = workbook
                .with_header_row(calamine::HeaderRow::Row(2))
                .worksheet_range(&sheet)
                .map_err(|error| Error::Sheet {
                    workbook: file.display().to_string(),
                    sheet,
                    error,
                })?;
            T::load(&sheet.rows().collect::<Vec<_>>()).await?;

            Ok(())
        }));
    }

    let mut tasks = Vec::new();

    load::<LinkTestData>(data_dir.join("link_test.ods"), "LinkTest", &mut tasks);
    load::<item::RandomBoxData>(data_dir.join("item/random_box.ods"), "RandomBox", &mut tasks);
    load::<item::WeaponData>(data_dir.join("item/weapon.ods"), "Weapon", &mut tasks);

    for task in tasks {
        match task.await {
            Ok(result) => result?,
            _  => panic!("Data loading task has failed!"),
        }
    }
    Ok(())
}

async fn init_concrete_tables() -> Result<(), Error> {
    fn init<T: crate::Loadable>(
        tasks: &mut Vec<tokio::task::JoinHandle<Result<(), Error>>>,
    ) {
        tasks.push(tokio::task::spawn(async move {
            T::init()?;
            Ok(())
        }));
    }

    let mut tasks = Vec::new();

    init::<LinkTestData>(&mut tasks);
    init::<item::RandomBoxData>(&mut tasks);
    init::<item::WeaponData>(&mut tasks);

    for task in tasks {
        match task.await {
            Ok(result) => result?,
            _  => panic!("Data initializing task has failed!"),
        }
    }
    Ok(())
}
