use crate::bikes::schema::bikes;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;
use diesel::prelude::*;
use super::BikeModel;

use crate::application::provider::{SQLError,get_connection} ;

// It is the bike representation.
pub struct BikeReaderStorage;


impl BikeReaderStorage {

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
}
