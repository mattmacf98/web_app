use actix_web::{HttpResponse, web};
use crate::database::{DB};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::schema::to_do;
use crate::models::item::item::Item;
use crate::diesel;
use crate::diesel::prelude::*;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken, db:DB) -> HttpResponse {
    let items = to_do::table
        .filter(to_do::columns::title.eq_all(to_do_item.title.as_str()))
        .filter(to_do::columns::user_id.eq_all(token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&db.connection);
    return HttpResponse::Ok().json(ToDoItems::get_state(Some(token.user_id.clone())));
}
