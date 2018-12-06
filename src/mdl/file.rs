use crate::schema::{file_assocs, files};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

enum_column! {
    #[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
    pub enum FileKind {
        File = 0,
        Dir = 1,
    }
}

#[derive(Debug, Queryable, Serialize)]
pub struct File {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub kind: FileKind,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub kind: FileKind,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "file_assocs"]
pub struct NewFileAssoc {
    pub dir_id: i64,
    pub child_id: i64,
    pub child_name: String,
}
