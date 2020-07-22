use diesel::prelude::*;
use uuid::Uuid;

use super::bike_model::BikeModel;
use crate::application::provider::{get_connection, SQLError};
use crate::bikes::ports::BikeWriter;
use crate::bikes::schema::bikes;
use crate::bikes::Bike;

pub struct BikeWriterStorage;

impl BikeWriterStorage {
    pub fn new() -> Self {
        BikeWriterStorage {}
    }

    fn create(bike: BikeModel) -> Result<BikeModel, SQLError> {
        let connection = get_connection()?;
        let bicycle = diesel::insert_into(bikes::table)
            .values(bike)
            .get_result(&connection)?;
        Ok(bicycle)
    }

    fn update(id: Uuid, bicycle: BikeModel) -> Result<BikeModel, SQLError> {
        let connection = get_connection()?;
        let bicycle = diesel::update(bikes::table)
            .filter(bikes::id.eq(id))
            .set(bicycle)
            .get_result(&connection)?;
        Ok(bicycle)
    }

    fn delete(id: Uuid) -> Result<usize, SQLError> {
        let connection = get_connection()?;
        let affected_rows =
            diesel::delete(bikes::table.filter(bikes::id.eq(id))).execute(&connection)?;

        Ok(affected_rows)
    }
}

impl BikeWriter for BikeWriterStorage {
    fn create(&self, bike: Bike) -> Bike {
        let bike_model = BikeModel::from(bike);
        let bicycle = BikeWriterStorage::create(bike_model).unwrap();
        bicycle.to_domain()
    }
}
