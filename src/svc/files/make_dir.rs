use crate::mdl::{self, File, FileKind, User};
use crate::svc::tree::ModifyDir;
use crate::{db, prelude::*};
use diesel::Connection;
use serde_json::json;

register_service!(MakeDir);

pub trait MakeDir: ModifyDir + db::HaveConn {
    fn make_dir(&self, mut user: User, path: &str, name: String) -> Result<File> {
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

            let parent_id = self.modify_dir(&mut user.tree, path, |obj| {
                obj[&new_file.name] = json!({ "..id": new_file.id });
                let parent_id = obj["..id"].as_i64().unwrap();
                Ok(parent_id)
            })?;

            db::files::associate(
                conn,
                &mdl::NewFileAssoc {
                    dir_id: parent_id,
                    child_id: new_file.id,
                    child_name: new_file.name.clone(),
                },
            )?;

            db::users::update_tree(conn, user.id, &user.tree)?;

            Ok(new_file)
        })
    }
}
