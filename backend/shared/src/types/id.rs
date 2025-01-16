use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Auth0Id(String);

impl From<String> for Auth0Id {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl Auth0Id {
    pub fn id(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ClassId(Uuid);

impl From<Uuid> for ClassId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl ClassId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}
