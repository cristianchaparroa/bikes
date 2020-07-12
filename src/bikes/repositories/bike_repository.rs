// BikeWriter is in charge to write information related
// to Bikes
pub trait BikeWriter {   

    // It persists for first time a bike 
    pub fn create(bike:Bike);
}