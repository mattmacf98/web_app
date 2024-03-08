use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use actix_service::Service;

mod jwt;
mod to_do;
mod views;
mod state;
mod processes;
mod json_serialization;

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
