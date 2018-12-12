mod error;
mod files;

use crate::store::AppStore;
use actix_web::middleware::Logger;
use actix_web::{App, HttpRequest};

fn ping(_req: &HttpRequest<AppStore>) -> &'static str {
    "Pong"
}

pub fn create_app(store: AppStore) -> App<AppStore> {
    App::with_state(store)
        .middleware(Logger::default())
        .resource("/_ping", |r| r.f(ping))
        .scope("/api", |scope| {
            scope
                .resource("files/file/{path:.*}", |r| {
                    r.get().with(files::get);
                    r.post().with(files::create);
                    r.delete().with(files::delete)
                })
                .resource("files/move/{path:.*}", |r| {
                    r.post().with(files::moves);
                })
                .resource("files/all/{path:.*}", |r| {
                    r.get().with(files::all);
                })
        })
}
