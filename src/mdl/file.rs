use crate::schema::{file_assocs, files};
use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub kind: i16,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub kind: i16,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "file_assocs"]
pub struct NewFileAssoc {
    pub dir_id: i64,
    pub child_id: i64,
    pub child_name: String,
}
