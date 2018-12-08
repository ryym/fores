use crate::mdl;
use crate::prelude::*;
use serde_json::Value as JsonValue;

register_service!(FindDir);

pub trait FindDir {
    fn find_dir(&self, user: &mdl::User, path: &str) -> Result<i64> {
        let keys = path.split("/").filter(|s| s.len() > 0).collect::<Vec<_>>();
        find_dir(&user.tree, keys.into_iter())
    }
}

fn find_dir<'a, I>(tree: &JsonValue, mut keys: I) -> Result<i64>
where
    I: Iterator<Item = &'a str>,
{
    match keys.next() {
        Some(key) => match tree.get(key) {
            Some(inner) => find_dir(inner, keys),
            None => Err(ErrorKind::Misc("invalid path".to_owned()).into()),
        },
        None => {
            let dir_id = serde_json::from_value(tree["..id"].clone()).unwrap();
            Ok(dir_id)
        }
    }
}
