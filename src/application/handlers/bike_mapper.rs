use crate::application::handlers::{BikeRequest, BikeResponse};
use crate::bikes::Bike;
use chrono::Utc;
use uuid::Uuid;

pub struct BikeMapper;

impl BikeMapper {
    pub fn from_request(bike: BikeRequest) -> Bike {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        Bike {
            id: id,
            description: bike.description,
            model: bike.model,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn to_response(bike: Bike) -> BikeResponse {
        BikeResponse {
            id: bike.id,
            description: bike.description,
            model: bike.model,
            created_at: bike.created_at,
            updated_at: bike.updated_at,
        }
    }
}
