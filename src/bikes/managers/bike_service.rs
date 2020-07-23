use std::rc::Rc;
use uuid::Uuid;

use crate::bikes::ports::BikeManager;
use crate::bikes::ports::{BikeReader, BikeWriter};
use crate::bikes::Bike;
#[derive(Clone)]
pub struct BikeService {
    writer: Rc<dyn BikeWriter>,
    reader: Rc<dyn BikeReader>,
}

impl BikeService {
    pub fn new(w: Rc<dyn BikeWriter>, r: Rc<dyn BikeReader>) -> Self {
        BikeService {
            writer: w,
            reader: r,
        }
    }
}

impl BikeManager for BikeService {
    fn create(&self, bike: Bike) -> Bike {
        self.writer.create(bike)
    }

    fn get_all(&self) -> Vec<Bike> {
        self.reader.find_all()
    }

    fn get_by_id(&self, id: Uuid) -> Bike {
        self.reader.find_by_id(id)
    }
}
