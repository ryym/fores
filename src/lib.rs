// https://github.com/diesel-rs/diesel/issues/1785
// TODO: Remove this after diesel published 1.4.
#![allow(proc_macro_derive_resolution_fallback)]

// This is necessary to use the `table_name` attribute
// for model definitions.
#[macro_use]
extern crate diesel;

mod prelude {
    use crate::error::Error;
    use std::result;

    pub use crate::error::ErrorKind;
    pub use failure::{Fail, ResultExt};

    pub type Result<T, E = Error> = result::Result<T, E>;
}

#[macro_use]
mod store;

mod auth;
mod db;
pub mod error;
mod mdl;
mod schema;
mod svc;
mod web;

use actix_web::server;
use std::env;

pub fn run() -> prelude::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_url = must_get_env_var("DATABASE_URL");
    let db_pool = db::new_pool(db_url)?;

    let port = env::var("PORT").unwrap_or("3000".to_owned());
    log::info!("Starting server at 127.0.0.1:{}", port);

    server::new(move || {
        let store = store::AppStore::create(db_pool.clone());
        web::create_app(store)
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect("start server")
    .run();

    Ok(())
}

fn must_get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}
