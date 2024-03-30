use actix_cors::Cors;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate core;

use actix_web::{App, HttpServer, HttpResponse, middleware::Logger};
use env_logger;
use actix_service::Service;
use futures::future::{ok, Either};

mod jwt;
mod to_do;
mod views;
mod json_serialization;
mod database;
mod schema;
mod models;
mod config;
mod counter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let site_counter = counter::Counter{count: 0};
    site_counter.save();

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        App::new()
            .wrap_fn(|req, srv| {
                let end_result;
                let passed: bool = req.path().contains(&format!("/{}/", ALLOWED_VERSION));

                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                println!("{:?}", &site_counter);
                site_counter.save();

                if passed {
                    end_result = Either::Left(srv.call(req));
                } else {
                    let resp = HttpResponse::NotImplemented().body(format!("only {} API is supported", ALLOWED_VERSION));
                    end_result = Either::Right(ok(req.into_response(resp).map_into_boxed_body()));
                }

                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory).wrap(cors).wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
