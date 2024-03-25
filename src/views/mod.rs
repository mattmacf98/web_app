use actix_web::web::ServiceConfig;
use crate::views::app::app_views_factory;
use crate::views::auth::auth_view_factory;
use crate::views::to_do::to_do_view_factory;
use crate::views::users::user_views_factory;

mod auth;
mod to_do;
mod app;
mod users;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_view_factory(app);
    to_do_view_factory(app);
    app_views_factory(app);
    user_views_factory(app);
}
