use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::{Map, Value};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        None => return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title)),
        Some(result) => status = TaskStatus::from_string(result.as_str().unwrap().to_string())
    }

    let existing_item = to_do_factory(&to_do_item.title, status.clone());

    process_input(existing_item, "delete".to_string(), &state);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
