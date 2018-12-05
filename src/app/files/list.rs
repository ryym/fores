use crate::mdl::{File, User};
use crate::{db, prelude::*};
use diesel::prelude::*;
use serde_json::Value as JsonValue;

register_service!(ListFiles);

pub trait ListFiles: db::HaveConn {
    fn list_files(&self, user: &User, path: &str) -> Result<Vec<File>> {
        use crate::schema::{file_assocs, files};

        let dir_id = extract_dir_id(&user.tree, path)?;
        files::table
            .inner_join(file_assocs::table.on(file_assocs::child_id.eq(files::id)))
            .filter(file_assocs::dir_id.eq(dir_id))
            .select(files::table::all_columns())
            .get_results(self.conn())
            .map_err(|e| e.into())
    }
}

fn extract_dir_id(tree: &JsonValue, path: &str) -> Result<i64> {
    let keys = path.split("/").filter(|s| s.len() > 0).collect::<Vec<_>>();
    extract_recur(tree, keys.into_iter())
}

fn extract_recur<'a, I>(tree: &JsonValue, mut keys: I) -> Result<i64>
where
    I: Iterator<Item = &'a str>,
{
    match keys.next() {
        Some(key) => match tree.get(key) {
            Some(inner) => extract_recur(inner, keys),
            None => Err(ErrorKind::Misc("invalid path".to_owned()).into()),
        },
        None => {
            let dir_id = serde_json::from_value(tree["..id"].clone()).unwrap();
            Ok(dir_id)
        }
    }
}
