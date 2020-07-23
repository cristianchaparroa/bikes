use diesel::prelude::*;
use std::rc::Rc;
use uuid::Uuid;

use super::bike_model::BikeModel;
use crate::application::provider::{DbConnection, Pool};
use crate::bikes::ports::BikeReader;
use crate::bikes::schema::bikes;
use crate::bikes::Bike;

#[derive(Clone)]
pub struct BikeReaderStorage {
    pool: Rc<Pool>,
}

impl BikeReaderStorage {
    pub fn new(pool: Rc<Pool>) -> Self {
        BikeReaderStorage { pool }
    }

    pub fn get_connection(&self) -> DbConnection {
        self.pool
            .get()
            .expect("Failure retrieving the pooled connection")
    }
}

impl BikeReader for BikeReaderStorage {
    fn find_all(&self) -> Vec<Bike> {
        let connection = self.get_connection();

        let bicycles = bikes::table.load::<BikeModel>(&connection).unwrap();

        let bicycles = bicycles.into_iter().map(|b| b.to_domain()).collect();

        bicycles
    }

    fn find_by_id(&self, id: Uuid) -> Bike {
        let connection = self.get_connection();

        let bicycle: BikeModel = bikes::table
            .filter(bikes::id.eq(id))
            .first(&connection)
            .unwrap();

        bicycle.to_domain()
    }
}
