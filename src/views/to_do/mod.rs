use actix_web::web::{ServiceConfig, post, get, scope};

mod create;
mod get;
mod edit;
mod delete;

pub fn to_do_view_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/item")
        .route("create/{title}", post().to(create::create))
        .route("get", get().to(get::get))
        .route("edit", post().to(edit::edit))
        .route("delete", post().to(delete::delete))
    );
}
