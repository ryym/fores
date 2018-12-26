use crate::prelude::*;
use serde_json::Value as JsonValue;

mod find_dir;
mod for_each;
mod modify_dir;

pub use self::{find_dir::*, for_each::*, modify_dir::*};

fn path_to_vec(path: &str) -> Vec<&str> {
    // Filter empty strings ("".split("/") -> [""]).
    path.split('/').filter(|s| !s.is_empty()).collect()
}

pub fn split_path(path: &str) -> Result<(Vec<&str>, &str)> {
    split_path_opt(path).ok_or_else(|| Error::invalid("path is empty"))
}

pub fn split_path_opt(path: &str) -> Option<(Vec<&str>, &str)> {
    let mut keys = path_to_vec(path);
    keys.pop().map(|last| (keys, last))
}

pub fn get_dir_id(obj: &JsonValue) -> i64 {
    obj["..id"].as_i64().unwrap()
}
