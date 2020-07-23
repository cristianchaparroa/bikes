use crate::bikes::Bike;
use uuid::Uuid;
// BikeWriter is in charge to write information related
// to Bikes
pub trait BikeWriter {
    // It persists for first time a bike
    fn create(&self, bike: Bike) -> Bike;

    fn update(&self, id: Uuid, bicycle: Bike) -> Bike;

    fn delete(&self, id: Uuid) -> usize;
}

pub trait BikeReader {
    fn find_all(&self) -> Vec<Bike>;

    fn find_by_id(&self, id: Uuid) -> Bike;
}
