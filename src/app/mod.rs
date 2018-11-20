use actix_web::middleware::Logger;
use actix_web::{App, HttpRequest};
use crate::store::AppStore;

fn ping(_req: &HttpRequest<AppStore>) -> &'static str {
    "Pong"
}

pub fn create(store: AppStore) -> App<AppStore> {
    App::with_state(store)
        .middleware(Logger::default())
        .resource("/_ping", |r| r.f(ping))
}
