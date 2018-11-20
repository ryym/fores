use crate::db;
use crate::prelude::*;

pub trait Store {
    type Svc;
    fn service(&self) -> Result<Self::Svc>;
}

pub struct AppStore {
    db_pool: db::Pool,
}

impl AppStore {
    pub fn create(db_pool: db::Pool) -> AppStore {
        AppStore { db_pool }
    }
}

impl Store for AppStore {
    type Svc = Hub;
    fn service(&self) -> Result<Self::Svc> {
        let conn = db::get_conn(&self.db_pool)?;
        Ok(Hub { conn })
    }
}

pub struct Hub {
    conn: db::PooledConn,
}

impl db::HaveConn for Hub {
    fn conn(&self) -> &db::Conn {
        &self.conn
    }
}

macro_rules! register_service {
    ($trait:ident) => {
        impl $trait for crate::store::Hub {}
    };
}
