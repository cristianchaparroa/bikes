

// Bike Creator is in charge to manage the bussiness logic for 
// Bikes
pub trait BikeCreator {
    fn create(bike :Bike);
}


struct BikeService {
    repository : Bike
}

impl BikeCreator for BikeService {

    fn create(bike :Bike) {

    } 
}