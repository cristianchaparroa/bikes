use crate::bikes::repositories::BikeWriter;
use crate::bikes::Bike;

// Bike Creator is in charge to manage the bussiness logic for
// Bikes
pub trait BikeCreator {
    fn create(&self, bike: Bike) -> Bike;
}

pub struct BikeService {
    // https://stackoverflow.com/questions/26212397/references-to-traits-in-structs
    repository: Box<dyn BikeWriter>,
}

impl BikeService {
    pub fn new(r: Box<dyn BikeWriter>) -> Self {
        BikeService { repository: r }
    }
}

impl BikeCreator for BikeService {
    fn create(&self, bike: Bike) -> Bike {
        self.repository.create(bike)
    }
}
