use time::OffsetDateTime;
use uuid::Uuid;

pub struct OrgModel {
    id: Uuid,
    name: String,
    status: bool,
    created_at: OffsetDateTime,
    update_at: OffsetDateTime,
}

impl OrgModel {
    pub fn new() -> OrgModel {
        OrgModel {
            id: Uuid::new_v4(),
            name: String::new(),
            status: false,
            created_at: OffsetDateTime::now_utc(),
            update_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn set_id(&mut self, value: &Uuid) {
        self.id = *value;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.to_owned();
    }

    pub fn get_status(&self) -> bool {
        self.status
    }

    pub fn set_status(&mut self, value: bool) {
        self.status = value;
    }

    pub fn get_created_at(&self) -> &OffsetDateTime {
        &self.created_at
    }

    pub fn set_created_at(&mut self, value: &OffsetDateTime) {
        self.created_at = *value;
    }

    pub fn get_update_at(&self) -> &OffsetDateTime {
        &self.update_at
    }

    pub fn set_update_at(&mut self, value: &OffsetDateTime) {
        self.update_at = *value;
    }
}
