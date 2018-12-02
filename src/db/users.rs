use crate::schema::users;
use crate::{db, prelude::*};
use diesel::prelude::*;

pub fn update_tree(conn: &db::Conn, id: i64, tree: &serde_json::Value) -> Result<()> {
    diesel::update(users::table)
        .filter(users::id.eq(id))
        .set(users::tree.eq(tree))
        .execute(conn)?;
    Ok(())
}
