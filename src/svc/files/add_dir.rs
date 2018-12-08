use crate::mdl::{self, File, FileKind, User};
use crate::{db, prelude::*};
use diesel::Connection;
use serde_json::{json, Value as JsonValue};
use std::mem;

register_service!(AddDir);

pub trait AddDir: db::HaveConn {
    fn add_dir(&self, user: User, path: &str, name: String) -> Result<File> {
        let conn = self.conn();
        conn.transaction(|| {
            let new_file = db::files::insert(
                conn,
                user.id,
                &mdl::NewFile {
                    kind: FileKind::Dir,
                    name,
                },
            )?;

            let (new_tree, parent_id) = add_dir(user.tree, path, &new_file)?;

            db::files::associate(
                conn,
                &mdl::NewFileAssoc {
                    dir_id: parent_id,
                    child_id: new_file.id,
                    child_name: new_file.name.clone(),
                },
            )?;

            db::users::update_tree(conn, user.id, &new_tree)?;

            Ok(new_file)
        })
    }
}

fn add_dir(tree: JsonValue, path: &str, file: &File) -> Result<(JsonValue, i64)> {
    let keys = path.split("/").filter(|s| s.len() > 0).collect::<Vec<_>>();
    add_dir_recur(tree, keys.into_iter(), &file)
}

fn add_dir_recur<'a, I>(mut obj: JsonValue, mut keys: I, file: &File) -> Result<(JsonValue, i64)>
where
    I: Iterator<Item = &'a str>,
{
    match keys.next() {
        Some(key) => {
            let inner = match obj.get_mut(key) {
                Some(inner) => inner,
                None => return Err(ErrorKind::Misc("invalid path".to_owned()).into()),
            };
            let child = mem::replace(inner, JsonValue::Null);
            let (child, parent_id) = add_dir_recur(child, keys, file)?;
            mem::replace(inner, child);
            Ok((obj, parent_id))
        }
        None => {
            obj[&file.name] = json!({ "..id": file.id });
            let dir_id = serde_json::from_value(obj["..id"].clone()).unwrap();
            Ok((obj, dir_id))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn add_new_dir() -> Result<()> {
        let json = json!({
            "a": {
                "..id": 1,
                "b": {
                    "..id": 2,
                    "x": {
                        "..id": 10,
                    }
                }
            }
        });

        let created_at = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);
        let new_file = File {
            id: 3,
            created_at,
            updated_at: created_at,
            kind: FileKind::File,
            name: "c".to_owned(),
        };
        let (json, dir_id) = add_dir(json, "a/b", &new_file)?;

        let expected = json!({
            "a": {
                "..id": 1,
                "b": {
                    "..id": 2,
                    "c": {
                        "..id": 3,
                    },
                    "x": {
                        "..id": 10,
                    }
                }
            }
        });
        assert_eq!((json, dir_id), (expected, 2));

        Ok(())
    }
}
