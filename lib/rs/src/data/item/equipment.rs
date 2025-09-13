// This is a generated file. DO NOT MODIFY.
use tracing::info;
use crate::DataId;

pub static EQUIPMENT_DATA: tokio::sync::OnceCell<EquipmentData> = tokio::sync::OnceCell::const_new();

#[derive(Debug)]
pub struct Equipment {
    pub kind: crate::item::EquipmentKind,
}

impl Equipment {
    fn parse(row: &[calamine::Data]) -> Result<(DataId, Self), crate::LoadError> {
        let kind = crate::item::EquipmentKind::parse(&row[0])?;

        Ok((id, Self {
            kind,
        }))
    }
}

impl crate::Linkable for Equipment {
    fn get(id: DataId) -> Option<&'static Self> {
        EquipmentData::get(id)
    }
}

pub struct EquipmentData {
    data: std::collections::HashMap<DataId, Equipment>,
}

impl EquipmentData {
    pub fn get(id: DataId) -> Option<&'static Equipment> {
        EQUIPMENT_DATA
            .get()
            .expect("EQUIPMENT_DATA is not initialized yet")
            .data
            .get(&id)
    }
}

impl crate::Loadable for EquipmentData {
    fn load(rows: &[&[calamine::Data]]) -> Result<(), crate::LoadError> {
        let mut objects = std::collections::HashMap::new();
        for row in rows {
            let (id, object) = Equipment::parse(row)?;

            if objects.contains_key(&id) {
                return Err(crate::LoadError::DuplicatedId {
                    type_name: std::any::type_name::<Equipment>(),
                    id,
                });
            }

            objects.insert(id, object);
        }

        if !EQUIPMENT_DATA.set(Self { data: objects }).is_ok() {
            return Err(crate::LoadError::AlreadyLoaded {
                type_name: std::any::type_name::<Equipment>(),
            });
        }

        info!("Loaded {} rows", rows.len());
        Ok(())
    }
}
