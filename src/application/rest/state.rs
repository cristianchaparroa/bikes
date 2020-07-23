use std::rc::Rc;
use std::sync::Arc;

use crate::application::adapters::repository::{BikeReaderStorage, BikeWriterStorage};
use crate::application::handlers::BikeHandler;
use crate::application::provider::Postgres;
use crate::bikes::managers::BikeService;

#[derive(Clone)]
pub struct State {
    pub bike_handler: BikeHandler,
}

impl State {
    pub fn new() -> Self {
        let postgres = Postgres::new();
        let pool = Rc::new(postgres.pool);

        let writer_storage = BikeWriterStorage::new(Rc::clone(&pool));
        let reader_storage = BikeReaderStorage::new(Rc::clone(&pool));

        let bike_writer = Rc::new(writer_storage);
        let bike_reader = Rc::new(reader_storage);
        let bike_service = BikeService::new(bike_writer, bike_reader);
        let bike_service = Arc::new(bike_service);
        let bike_handler = BikeHandler::new(bike_service);

        State { bike_handler }
    }
}
