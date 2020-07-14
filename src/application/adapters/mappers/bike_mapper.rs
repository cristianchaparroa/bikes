use super::super::models::BikeModel;
use crate::bikes::Bike;

pub struct BikeMapper;

impl BikeMapper {
    pub fn to_model(bike: Bike) -> BikeModel {
        BikeModel {
            id: bike.id,
            description: bike.description,
            model: bike.model,
            created_at: bike.created_at,
            updated_at: bike.updated_at,
        }
    }

    pub fn to_domain(bike: BikeModel) -> Bike {
        Bike {
            id: bike.id,
            description: bike.description,
            model: bike.model,
            created_at: bike.created_at,
            updated_at: bike.updated_at,
        }
    }
}
