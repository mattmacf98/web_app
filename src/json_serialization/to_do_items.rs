use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use diesel;
use diesel::prelude::*;
use serde::{Serialize};
use crate::database::{DBCONNECTION, establish_connection};
use crate::to_do::enums::TaskStatus;
use crate::to_do::{ItemType, to_do_factory};
use crate::to_do::structs::base::Base;
use crate::schema::to_do;
use crate::models::item::item::Item;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemType>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemType::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemType::Done(packed) => done_array_buffer.push(packed.super_struct)
            }
        }

        let pending_count = pending_array_buffer.len() as i8;
        let done_count = done_array_buffer.len() as i8;

        return ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }

    pub fn get_state(user_id: Option<i32>) -> ToDoItems {
        let connection = DBCONNECTION.db_connection.get().unwrap();


        let items = match user_id {
            None => to_do::table.order(to_do::columns::id.asc()).load::<Item>(&connection).unwrap(),
            Some(user_id) => to_do::table.filter(to_do::columns::user_id.eq_all(user_id)).order(to_do::columns::id.asc()).load::<Item>(&connection).unwrap()
        };

        let mut array_buffer = Vec::with_capacity(items.len());
        for item in items {
            let status = TaskStatus::from_string(item.status);
            let item = to_do_factory(&item.title, status);
            array_buffer.push(item);
        }

        return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
