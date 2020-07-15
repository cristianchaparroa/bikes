#[macro_use]
extern crate diesel;

mod application;
mod bikes;

use application::provider;

use application::adapters::repository::{BikeReaderStorage, BikeWriterStorage};

use bikes::Bike;

use bikes::managers::BikeService;
use bikes::ports::BikeManager;

use dotenv::dotenv;

fn main() {
    //let postgres = Postgres::new();
    //let connection = postgres.conn;

    dotenv().ok();
    provider::init();

    let bike_writer = BikeWriterStorage::new();
    let bike_reader = BikeReaderStorage::new();

    let box_bike_writer = Box::new(bike_writer);
    let box_bike_reader = Box::new(bike_reader);

    let bike_service = BikeService::new(box_bike_writer, box_bike_reader);

    let bike = Bike::new("It's a montain bike", "BMT");
    bike_service.create(bike);
}
