use actix_web::{Responder, web};
use serde_json::{Map, Value};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::{ItemType, to_do_factory};

pub async fn get() -> impl Responder {
    return ToDoItems::get_state();
}
