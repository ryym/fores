use crate::prelude::*;
use serde_json::Value as JsonValue;

register_service!(ReplaceDir);

pub trait ReplaceDir {
    fn replace_dir<F, T>(&self, mut tree: &mut JsonValue, path: &str, f: F) -> Result<T>
    where
        F: FnOnce(&mut JsonValue) -> Result<T>,
    {
        let keys = path.split("/").filter(|s| s.len() > 0).collect::<Vec<_>>();
        replace_dir(&mut tree, keys.into_iter(), f)
    }
}

fn replace_dir<'a, I, F, T>(mut obj: &mut JsonValue, mut keys: I, f: F) -> Result<T>
where
    I: Iterator<Item = &'a str>,
    F: FnOnce(&mut JsonValue) -> Result<T>,
{
    match keys.next() {
        Some(key) => {
            let mut child = match obj.get_mut(key) {
                Some(child) => child,
                None => return Err(ErrorKind::Misc("invalid path".to_owned()).into()),
            };
            Ok(replace_dir(&mut child, keys, f)?)
        }
        None => Ok(f(&mut obj)?),
    }
}
