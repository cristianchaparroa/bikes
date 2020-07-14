use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
pub struct Bike {
    pub id: Uuid,
    pub description: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Bike {
    pub fn new(description: &str, model: &str) -> Self {
        let description = description.into();
        let model = model.into();
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        Bike {
            id: id,
            description: description,
            model: model,
            created_at: now,
            updated_at: now,
        }
    }
}
