use uuid::Uuid;

#[derive(Debug,)]
pub struct UserId(Uuid);

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl UserId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}
