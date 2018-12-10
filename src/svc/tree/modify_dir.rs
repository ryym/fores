use crate::prelude::*;
use serde_json::Value as JsonValue;

register_service!(ModifyDir);

pub trait ModifyDir {
    fn modify_dir<F, T>(&self, mut tree: &mut JsonValue, path: &str, f: F) -> Result<T>
    where
        F: FnOnce(&mut JsonValue) -> Result<T>,
    {
        let keys = super::path_to_vec(path);
        modify_dir(&mut tree, keys.into_iter(), f)
    }
}

fn modify_dir<'a, I, F, T>(mut obj: &mut JsonValue, mut keys: I, f: F) -> Result<T>
where
    I: Iterator<Item = &'a str>,
    F: FnOnce(&mut JsonValue) -> Result<T>,
{
    match keys.next() {
        Some(key) => {
            let mut child = match obj.get_mut(key) {
                Some(child) => child,
                None => return Err(Error::invalid("invalid path")),
            };
            Ok(modify_dir(&mut child, keys, f)?)
        }
        None => Ok(f(&mut obj)?),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn add_new_dir() -> Result<()> {
        let mut json = json!({
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

        struct Mock {}
        impl ModifyDir for Mock {}

        let mock = Mock {};
        let parent_id = mock.modify_dir(&mut json, "a/b", |obj| {
            obj["c"] = json!({ "..id": 3 });
            Ok(super::get_dir_id(obj))
        })?;

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
        assert_eq!(json, expected);
        assert_eq!(parent_id, 2, "must get b's id");

        Ok(())
    }
}
