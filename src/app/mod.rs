mod error;
mod files;

use crate::store::AppStore;
use actix_web::middleware::Logger;
use actix_web::{App, HttpRequest};

fn ping(_req: &HttpRequest<AppStore>) -> &'static str {
    "Pong"
}

pub fn create(store: AppStore) -> App<AppStore> {
    App::with_state(store)
        .middleware(Logger::default())
        .resource("/_ping", |r| r.f(ping))
        .scope("/api", |scope| {
            scope
                .resource("files", |r| r.post().with(files::add))
                .resource("files/list/{path:.*}", |r| r.get().with(files::list))
        })
}
