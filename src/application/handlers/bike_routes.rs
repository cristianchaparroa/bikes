use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::application::handlers::BikeRequest;
use crate::application::handlers::Error;
use crate::application::rest::State;

#[get("/bikes")]
async fn get_all(state: web::Data<State>) -> Result<HttpResponse, Error> {
    state.bike_handler.get_all()
}

#[get("/bikes/{id}")]
async fn get_by_id(state: web::Data<State>, req: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let id: Uuid = req.into_inner();
    state.bike_handler.get_by_id(id)
}

#[post("/bikes")]
async fn create(
    state: web::Data<State>,
    req: web::Json<BikeRequest>,
) -> Result<HttpResponse, Error> {
    let bike = req.into_inner();
    state.bike_handler.create(bike)
}

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config.service(get_all);
    config.service(get_by_id);
    config.service(create);
}
