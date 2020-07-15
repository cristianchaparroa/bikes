use uuid::Uuid;

use crate::bikes::ports::BikeManager;
use crate::bikes::ports::{BikeReader, BikeWriter};
use crate::bikes::Bike;

pub struct BikeService {
    // https://stackoverflow.com/questions/26212397/references-to-traits-in-structs
    writer: Box<dyn BikeWriter>,
    reader: Box<dyn BikeReader>,
}

impl BikeService {
    pub fn new(w: Box<dyn BikeWriter>, r: Box<dyn BikeReader>) -> Self {
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
