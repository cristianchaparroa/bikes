#[macro_use]
extern crate diesel;

mod application;
mod bikes;

use application::provider;

use application::adapters::repository::BikeWriterStorage;
use bikes::services::{BikeCreator, BikeService};
use bikes::Bike;

use dotenv::dotenv;

fn main() {
    //let postgres = Postgres::new();
    //let connection = postgres.conn;

    dotenv().ok();
    provider::init();

    let bike_writer = BikeWriterStorage::new();
    let box_bike_writer = Box::new(bike_writer);
    let bike_service = BikeService::new(box_bike_writer);

    let bike = Bike::new("It's a montain bike", "BMT");
    bike_service.create(bike);
}
