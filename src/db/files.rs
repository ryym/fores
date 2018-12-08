use crate::db;
use crate::mdl::{File, NewFile, NewFileAssoc, NewFileOwner};
use crate::prelude::*;
use crate::schema::{file_assocs, file_owners, files};
use diesel::prelude::*;

pub fn insert(conn: &db::Conn, owner_id: i64, file: &NewFile) -> Result<File> {
    conn.transaction(|| {
        let file = diesel::insert_into(files::table)
            .values(file)
            .get_result::<File>(conn)?;

        diesel::insert_into(file_owners::table)
            .values(NewFileOwner {
                owner_id,
                file_id: file.id,
            })
            .execute(conn)?;

        Ok(file)
    })
}

pub fn associate(conn: &db::Conn, assoc: &NewFileAssoc) -> Result<()> {
    diesel::insert_into(file_assocs::table)
        .values(assoc)
        .execute(conn)?;
    Ok(())
}

pub fn delete(conn: &db::Conn, file_id: i64) -> Result<()> {
    diesel::delete(files::table.filter(files::id.eq(file_id))).execute(conn)?;
    Ok(())
}

pub fn delete_children(conn: &db::Conn, dir_id: i64) -> Result<usize> {
    // Currently Diesel does not support DELETE with JOINs
    // so we need to fetch children beforehand.
    let child_ids = file_assocs::table
        .select(file_assocs::child_id)
        .filter(file_assocs::dir_id.eq(dir_id))
        .load::<i64>(conn)?;

    diesel::delete(files::table.filter(files::id.eq_any(child_ids)))
        .execute(conn)
        .map_err(Into::into)
}
