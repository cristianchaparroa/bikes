use diesel::prelude::*;
use uuid::Uuid;

use crate::bikes::schema::bikes;

use super::bike_mapper::BikeMapper;
use super::bike_model::BikeModel;
use crate::application::provider::{get_connection, SQLError};
use crate::bikes::ports::BikeReader;
use crate::bikes::Bike;

pub struct BikeReaderStorage;

impl BikeReaderStorage {
    pub fn new() -> Self {
        BikeReaderStorage {}
    }

    pub fn get_all() -> Result<Vec<BikeModel>, SQLError> {
        let connection = get_connection()?;
        let bicycles = bikes::table.load::<BikeModel>(&connection)?;
        Ok(bicycles)
    }

    pub fn get_by_id(id: Uuid) -> Result<BikeModel, SQLError> {
        let connection = get_connection()?;
        let bicycle = bikes::table.filter(bikes::id.eq(id)).first(&connection)?;
        Ok(bicycle)
    }
}

impl BikeReader for BikeReaderStorage {
    fn find_all(&self) -> Vec<Bike> {
        let bycicles = BikeReaderStorage::get_all().unwrap();
        let bycicles = bycicles
            .into_iter()
            .map(|b| BikeMapper::to_domain(b))
            .collect();
        bycicles
    }

    fn find_by_id(&self, id: Uuid) -> Bike {
        let bike = BikeReaderStorage::get_by_id(id).unwrap();
        BikeMapper::to_domain(bike)
    }
}
