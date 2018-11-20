// https://github.com/diesel-rs/diesel/issues/1785
// TODO: Remove this after diesel published 1.4.
#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate failure;

mod prelude {
    use std::result;

    pub use crate::error::{Error, ErrorKind};
    pub use failure::{Fail, ResultExt};

    pub type Result<T, E = Error> = result::Result<T, E>;
}

#[macro_use]
mod store;

mod app;
mod db;
pub mod error;

use actix_web::server;
use std::env;

pub fn run() -> prelude::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_url = must_get_env_var("DATABASE_URL");
    let db_pool = db::new_pool(db_url)?;

    let port = env::var("PORT").unwrap_or("3000".to_owned());
    println!("Starting server at 127.0.0.1:{}", port);

    server::new(move || {
        let store = hub::AppStore::create(db_pool.clone());
        app::create(store)
    }).bind(format!("127.0.0.1:{}", port))
    .expect("start server")
    .run();

    Ok(())
}

fn must_get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}
