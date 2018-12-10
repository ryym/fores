use crate::db;
use crate::mdl::{File, FileKind, NewFile, NewFileAssoc, NewFileContent, NewFileOwner};
use crate::prelude::*;
use crate::schema::{file_assocs, file_contents, file_owners, files};
use diesel::prelude::*;

#[derive(Debug)]
pub struct InsertFile {
    pub owner_id: i64,
    pub name: String,
    pub content: String,
}

pub fn insert_file(conn: &db::Conn, form: InsertFile) -> Result<File> {
    conn.transaction(|| {
        let file = NewFile {
            kind: FileKind::File,
            name: form.name,
        };
        let file = diesel::insert_into(files::table)
            .values(file)
            .get_result::<File>(conn)?;

        let owner = NewFileOwner {
            owner_id: form.owner_id,
            file_id: file.id,
        };
        insert_owner(conn, &owner)?;

        let content = NewFileContent {
            file_id: file.id,
            content: form.content,
        };
        insert_content(conn, &content)?;

        Ok(file)
    })
}

#[derive(Debug)]
pub struct InsertDir {
    pub owner_id: i64,
    pub name: String,
}

pub fn insert_dir(conn: &db::Conn, form: InsertDir) -> Result<File> {
    conn.transaction(|| {
        let file = NewFile {
            kind: FileKind::Dir,
            name: form.name,
        };
        let file = diesel::insert_into(files::table)
            .values(file)
            .get_result::<File>(conn)?;

        let owner = NewFileOwner {
            owner_id: form.owner_id,
            file_id: file.id,
        };
        insert_owner(conn, &owner)?;

        Ok(file)
    })
}

fn insert_owner(conn: &db::Conn, new_owner: &NewFileOwner) -> Result<()> {
    diesel::insert_into(file_owners::table)
        .values(new_owner)
        .execute(conn)?;
    Ok(())
}

fn insert_content(conn: &db::Conn, new_content: &NewFileContent) -> Result<()> {
    diesel::insert_into(file_contents::table)
        .values(new_content)
        .execute(conn)?;
    Ok(())
}

pub fn associate(conn: &db::Conn, parent_id: i64, file: &File) -> Result<()> {
    diesel::insert_into(file_assocs::table)
        .values(NewFileAssoc {
            dir_id: parent_id,
            child_id: file.id,
            child_name: file.name.clone(),
        })
        .execute(conn)?;
    Ok(())
}

#[derive(Debug)]
pub struct Dessociate {
    pub parent_id: i64,
    pub child_id: i64,
}

pub fn dessociate(conn: &db::Conn, form: &Dessociate) -> Result<()> {
    let q = file_assocs::table
        .filter(file_assocs::dir_id.eq(form.parent_id))
        .filter(file_assocs::child_id.eq(form.child_id));
    diesel::delete(q).execute(conn)?;
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
