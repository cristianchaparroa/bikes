use crate::bikes::schema::bikes;
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
