use crate::bikes::schema::bikes;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use crate::application::provider::{get_connection, SQLError};

// It is the bike representation.
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "bikes"]
pub struct BikeModel {
    pub id: Uuid,
    pub description: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl BikeModel {
    pub fn new(description: &str, model: &str) -> Self {
        BikeModel {
            id: Uuid::new_v4(),
            description: description.into(),
            model: model.into(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl Display for BikeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BikeModel[id:{}, description:{}]",
            self.id, self.description
        )
    }
}

impl BikeModel {
    pub fn find_all() -> Result<Vec<Self>, SQLError> {
        let connection = get_connection()?;
        let bicycles = bikes::table.load::<BikeModel>(&connection)?;
        Ok(bicycles)
    }

    pub fn find_by_id(id: Uuid) -> Result<Self, SQLError> {
        let connection = get_connection()?;
        let bicycle = bikes::table.filter(bikes::id.eq(id)).first(&connection)?;
        Ok(bicycle)
    }

    pub fn create(bike: BikeModel) -> Result<Self, SQLError> {
        let connection = get_connection()?;
        let bicycle = diesel::insert_into(bikes::table)
            .values(bike)
            .get_result(&connection)?;
        Ok(bicycle)
    }

    pub fn update(id: Uuid, bicycle: BikeModel) -> Result<Self, SQLError> {
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
