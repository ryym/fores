use crate::mdl::{File, User};
use crate::svc::tree::{get_dir_id, ModifyDir};
use crate::{db, prelude::*};
use diesel::Connection;
use serde_json::json;

register_service!(MakeDir);

pub trait MakeDir: ModifyDir + db::HaveConn {
    fn make_dir(&self, mut user: User, path: &str, name: String) -> Result<File> {
        let conn = self.conn();
        conn.transaction(|| {
            let new_file = db::files::insert_dir(
                conn,
                db::files::InsertDir {
                    owner_id: user.id,
                    name,
                },
            )?;

            let parent_id = self.modify_dir(&mut user.tree, path, |obj| {
                obj[&new_file.name] = json!({ "..id": new_file.id });
                Ok(get_dir_id(obj))
            })?;

            db::files::associate(conn, parent_id, &new_file)?;
            db::users::update_tree(conn, user.id, &user.tree)?;

            Ok(new_file)
        })
    }
}
