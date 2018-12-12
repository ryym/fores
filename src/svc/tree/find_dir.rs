use crate::mdl;
use crate::prelude::*;
use serde_json::Value as JsonValue;

register_service!(FindDir);
register_service!(FindDirIdImpl);

pub trait FindDir {
    fn find_dir<'a>(&self, user: &'a mdl::User, path: &str) -> Result<&'a JsonValue> {
        let keys = super::path_to_vec(path);
        find_dir(&user.tree, keys.into_iter())
    }
}

fn find_dir<'a, I>(tree: &JsonValue, mut keys: I) -> Result<&JsonValue>
where
    I: Iterator<Item = &'a str>,
{
    match keys.next() {
        Some(key) => match tree.get(key) {
            Some(inner) => find_dir(inner, keys),
            None => Err(Error::invalid("invalid path")),
        },
        None => Ok(tree),
    }
}

pub trait FindDirId {
    fn find_dir_id(&self, user: &mdl::User, path: &str) -> Result<i64>;
}

pub trait FindDirIdImpl: FindDir {}
impl<T: FindDirIdImpl> FindDirId for T {
    fn find_dir_id(&self, user: &mdl::User, path: &str) -> Result<i64> {
        let dir = self.find_dir(user, path)?;
        Ok(super::get_dir_id(dir))
    }
}
