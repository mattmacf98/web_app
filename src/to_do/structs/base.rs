use serde::Serialize;
use super::super::enums::TaskStatus;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus
}
