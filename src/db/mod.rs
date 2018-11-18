use crate::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub type Conn = diesel::pg::PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub fn new_pool<S: Into<String>>(db_url: S) -> Result<Pool> {
    let manager = ConnectionManager::<Conn>::new(db_url.into());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .context("build DB pool")?;
    Ok(pool)
}

pub trait HaveConn {
    fn conn(&self) -> &Conn;
}

pub fn get_conn(pool: &Pool) -> Result<PooledConn> {
    let conn = pool.get().context("obtain DB connection")?;
    Ok(conn)
}
