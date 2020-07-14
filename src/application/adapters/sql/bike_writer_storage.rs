use super::BikeModel;
use crate::bikes::schema::bikes;
use diesel::prelude::*;
use uuid::Uuid;

use crate::application::provider::{get_connection, SQLError};

// It is the bike representation.
pub struct BikeWriterStorage;

impl BikeWriterStorage {
    pub fn create(bike: BikeModel) -> Result<BikeModel, SQLError> {
        let connection = get_connection()?;
        let bicycle = diesel::insert_into(bikes::table)
            .values(bike)
            .get_result(&connection)?;
        Ok(bicycle)
    }

    pub fn update(id: Uuid, bicycle: BikeModel) -> Result<BikeModel, SQLError> {
        let connection = get_connection()?;
        let bicycle = diesel::update(bikes::table)
            .filter(bikes::id.eq(id))
            .set(bicycle)
            .get_result(&connection)?;
        Ok(bicycle)
    }

    pub fn delete(id: Uuid) -> Result<usize, SQLError> {
        let connection = get_connection()?;
        let affected_rows =
            diesel::delete(bikes::table.filter(bikes::id.eq(id))).execute(&connection)?;

        Ok(affected_rows)
    }
}
