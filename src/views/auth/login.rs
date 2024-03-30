use std::collections::HashMap;
use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::json_serialization::login_response::LoginResponse;
use crate::schema::users;
use crate::jwt::JwToken;

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
   let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq_all(credentials.username.clone()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap()
    }

    return match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            let response = LoginResponse{token: raw_token.clone()};
            HttpResponse::Ok().append_header(("token", raw_token)).json(&response)
        },
        false => HttpResponse::Unauthorized().finish()
    };
}
