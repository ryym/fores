use crate::db;
use crate::mdl::{File, NewFile, NewFileAssoc};
use crate::prelude::*;
use crate::schema::{file_assocs, files};
use diesel::prelude::*;

pub fn insert(conn: &db::Conn, file: &NewFile) -> Result<File> {
    let file = diesel::insert_into(files::table)
        .values(file)
        .get_result::<File>(conn)?;
    Ok(file)
}

pub fn associate(conn: &db::Conn, assoc: &NewFileAssoc) -> Result<()> {
    diesel::insert_into(file_assocs::table)
        .values(assoc)
        .execute(conn)?;
    Ok(())
}
