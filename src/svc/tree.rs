mod find_dir;
mod modify_dir;

pub use self::{find_dir::*, modify_dir::*};

fn path_to_vec(path: &str) -> Vec<&str> {
    // Filter empty strings ("".split("/") -> [""]).
    path.split("/").filter(|s| s.len() > 0).collect()
}

pub fn split_path(path: &str) -> Option<(Vec<&str>, &str)> {
    let mut keys = path_to_vec(path);
    keys.pop().map(|last| (keys, last))
}
