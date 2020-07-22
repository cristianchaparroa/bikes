use crate::bikes::schema::bikes;
use crate::bikes::Bike;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

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

    pub fn from(bike: Bike) -> BikeModel {
        BikeModel {
            id: bike.id,
            description: bike.description.clone(),
            model: bike.model.clone(),
            created_at: bike.created_at,
            updated_at: bike.updated_at,
        }
    }

    pub fn to_domain(&self) -> Bike {
        Bike {
            id: self.id,
            description: self.description.clone(),
            model: self.model.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
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
