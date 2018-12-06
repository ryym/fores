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
