use diesel::prelude::*;
use std::rc::Rc;
use uuid::Uuid;

use super::bike_model::BikeModel;
use crate::application::provider::{DbConnection, Pool};
use crate::bikes::ports::BikeWriter;
use crate::bikes::schema::bikes;
use crate::bikes::Bike;

#[derive(Clone)]
pub struct BikeWriterStorage {
    pool: Rc<Pool>,
}

impl BikeWriterStorage {
    pub fn new(pool: Rc<Pool>) -> Self {
        BikeWriterStorage { pool }
    }

    pub fn get_connection(&self) -> DbConnection {
        self.pool
            .get()
            .expect("Failure retrieving the pooled connection")
    }
}

impl BikeWriter for BikeWriterStorage {
    fn create(&self, bike: Bike) -> Bike {
        let conn = self.get_connection();
        let bike_model = BikeModel::from(bike);

        let bicyle: BikeModel = diesel::insert_into(bikes::table)
            .values(bike_model)
            .get_result(&conn)
            .unwrap();

        bicyle.to_domain()
    }

    fn update(&self, id: Uuid, bicycle: Bike) -> Bike {
        let connection = self.get_connection();
        let bike_model = BikeModel::from(bicycle);

        let bike: BikeModel = diesel::update(bikes::table)
            .filter(bikes::id.eq(id))
            .set(bike_model)
            .get_result(&connection)
            .unwrap();

        bike.to_domain()
    }

    fn delete(&self, id: Uuid) -> usize {
        let connection = self.get_connection();

        let affected_rows = diesel::delete(bikes::table.filter(bikes::id.eq(id)))
            .execute(&connection)
            .unwrap();

        affected_rows
    }
}
