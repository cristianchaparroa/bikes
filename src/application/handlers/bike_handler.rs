use std::rc::Rc;
use uuid::Uuid;

use crate::application::handlers::Error;
use crate::application::handlers::{BikeMapper, BikeRequest};
use crate::bikes::ports::BikeManager;
use actix_web::HttpResponse;
#[derive(Clone)]
pub struct BikeHandler {
    service: Rc<dyn BikeManager>,
}

impl BikeHandler {
    pub fn new(service: Rc<dyn BikeManager>) -> Self {
        BikeHandler { service }
    }

    pub fn get_all(&self) -> Result<HttpResponse, Error> {
        let bikes = self.service.get_all();
        let response = HttpResponse::Ok().json(bikes);
        Ok(response)
    }

    pub fn get_by_id(&self, id: Uuid) -> Result<HttpResponse, Error> {
        let bike = self.service.get_by_id(id);
        let response = HttpResponse::Ok().json(bike);
        Ok(response)
    }

    pub fn create(&self, bike: BikeRequest) -> Result<HttpResponse, Error> {
        let bike = BikeMapper::from_request(bike);
        let bike = self.service.create(bike);
        let bike_response = BikeMapper::to_response(bike);
        let response = HttpResponse::Ok().json(bike_response);
        Ok(response)
    }
}
