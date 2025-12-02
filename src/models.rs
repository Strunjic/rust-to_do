use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToDo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
}

impl ToDo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            created_at: Utc::now(),
            completed: false,
        }
    }
}