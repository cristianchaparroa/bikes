use crate::bikes::Bike;
use uuid::Uuid;
// Bike Creator is in charge to manage the bussiness logic for
// Bikes
pub trait BikeManager {
    fn create(&self, bike: Bike) -> Bike;
    fn get_all(&self) -> Vec<Bike>;
    fn get_by_id(&self, id: Uuid) -> Bike;
}
