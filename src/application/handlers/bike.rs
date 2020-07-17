use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct BikeRequest {
    pub description: String,
    pub model: String,
}

#[derive(Serialize, Deserialize)]
pub struct BikeResponse {
    pub id: Uuid,
    pub description: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
