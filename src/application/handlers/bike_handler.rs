use actix_web::{get, post, web, HttpResponse};
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::application::adapters::repository::{BikeReaderStorage, BikeWriterStorage};
use crate::application::handlers::Error;
use crate::application::handlers::{BikeMapper, BikeRequest};
use crate::bikes::managers::BikeService;
use crate::bikes::ports::BikeManager;

lazy_static! {
    static ref BIKE_HANDLER: BikeHandler = BikeHandler::new();
}

pub fn new_bike_handler() {
    lazy_static::initialize(&BIKE_HANDLER);
}

struct BikeHandler {
    pub service: Box<dyn BikeManager + Sync>,
}
impl BikeHandler {
    fn new() -> Self {
        let bike_writer = BikeWriterStorage::new();
        let bike_reader = BikeReaderStorage::new();

        let box_bike_writer = Box::new(bike_writer);
        let box_bike_reader = Box::new(bike_reader);

        let bike_service = BikeService::new(box_bike_writer, box_bike_reader);
        let box_bike_service = Box::new(bike_service);

        BikeHandler {
            service: box_bike_service,
        }
    }
}

#[get("/bikes")]
async fn get_all() -> Result<HttpResponse, Error> {
    let bikes = BIKE_HANDLER.service.get_all();
    let response = HttpResponse::Ok().json(bikes);
    Ok(response)
}

#[get("/bikes/{id}")]
async fn get_by_id(req: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let id: Uuid = req.into_inner();
    let bike = BIKE_HANDLER.service.get_by_id(id);
    let response = HttpResponse::Ok().json(bike);
    Ok(response)
}

#[post("/bikes")]
async fn create(req: web::Json<BikeRequest>) -> Result<HttpResponse, Error> {
    let bike = req.into_inner();
    let bike = BikeMapper::from_request(bike);
    let bike = BIKE_HANDLER.service.create(bike);
    let bike_response = BikeMapper::to_response(bike);
    let response = HttpResponse::Ok().json(bike_response);
    Ok(response)
}

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config.service(get_all);
    config.service(get_by_id);
    config.service(create);
}
