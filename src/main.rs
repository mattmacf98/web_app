use actix_cors::Cors;
#[macro_use] extern crate diesel;
extern crate dotenv;
use actix_web::{App, HttpServer};
use actix_service::Service;

mod jwt;
mod to_do;
mod views;
mod json_serialization;
mod database;
mod schema;
mod models;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory).wrap(cors)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
