use actix_web::{HttpResponse, web};
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use crate::database::{DB};
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::schema::to_do;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    println!("here is the message in the token: {}", token.message);
    let results = to_do::table.filter(to_do::title.eq_all(&to_do_item.title));

    let _ = diesel::update(results)
        .set(to_do::status.eq_all("DONE"))
        .execute(&db.connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
