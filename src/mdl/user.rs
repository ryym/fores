use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub key: String,
    pub tree: serde_json::Value,
}
