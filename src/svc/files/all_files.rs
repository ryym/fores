use crate::svc::tree;
use crate::{db, mdl, prelude::*};
use std::collections::HashMap;

register_service!(AllFiles);

pub trait AllFiles: tree::FindDir + tree::ForEach + db::HaveConn {
    /// Get all files under the specified path, except directories.
    fn all_file_names(&self, user: &mdl::User, path: &str) -> Result<Vec<String>> {
        let mut dir_ids = vec![];
        let mut id_to_path = HashMap::new();

        let root = self.find_dir(user, path)?;
        self.for_each_tree(root, String::new(), |name, tree, path| {
            let id = tree::get_dir_id(tree);
            dir_ids.push(id);
            let path = path + name + "/";
            id_to_path.insert(id, path.clone());
            path
        });

        let parent_with_names = list_names(self.conn(), &dir_ids)?;

        let mut paths = parent_with_names
            .into_iter()
            .map(|(parent_id, name)| {
                let parent_path = &id_to_path[&parent_id];
                parent_path.clone() + &name
            })
            .collect::<Vec<_>>();

        paths.sort_unstable();
        Ok(paths)
    }
}

fn list_names(conn: &db::Conn, dir_ids: &[i64]) -> Result<Vec<(i64, String)>> {
    use crate::schema::file_assocs;
    use diesel::prelude::*;

    file_assocs::table
        .filter(file_assocs::dir_id.eq_any(dir_ids))
        .select((file_assocs::dir_id, file_assocs::child_name))
        .get_results::<(i64, String)>(conn)
        .map_err(Into::into)
}
