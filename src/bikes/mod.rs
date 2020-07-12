mod repositories;
mod services;
mod bike;


// It exposes the domain object Bike.
pub use bike::Bike;
pub use repositories::BikeWriter;
pub use services::BikeCreator;
