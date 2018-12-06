use crate::schema::file_owners;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub key: String,
    pub tree: serde_json::Value,
}

#[derive(Debug, Insertable)]
#[table_name = "file_owners"]
pub struct NewFileOwner {
    pub owner_id: i64,
    pub file_id: i64,
}
