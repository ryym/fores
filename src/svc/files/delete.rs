use crate::svc::tree::{get_dir_id, split_path, FindDirId, ModifyDir};
use crate::{db, mdl, prelude::*};
use diesel::prelude::*;

register_service!(DeleteDir);
register_service!(DeleteFile);

pub trait DeleteDir: ModifyDir + db::HaveConn {
    fn delete_dir(&self, mut user: mdl::User, path: &str) -> Result<()> {
        let (keys, dir_name) = split_path(path)?;
        let parent_path = keys.join("/");

        let dir_id = self.modify_dir(&mut user.tree, &parent_path, |parent| {
            let parent = parent.as_object_mut().unwrap();
            let obj = match parent.get(dir_name) {
                Some(obj) => obj,
                None => return Err(Error::invalid("invalid path")),
            };
            let dir_id = get_dir_id(obj);
            parent.remove(dir_name);
            Ok(dir_id)
        })?;

        let conn = self.conn();
        conn.transaction(|| {
            db::files::delete_children(conn, dir_id)?;
            db::files::delete(conn, dir_id)?;
            db::users::update_tree(conn, user.id, &user.tree)?;
            Ok(())
        })
    }
}

pub trait DeleteFile: FindDirId + db::HaveConn {
    fn delete_file(&self, user: &mdl::User, path: &str) -> Result<()> {
        use crate::schema::files;

        let (keys, file_name) = split_path(path)?;
        let dir_id = self.find_dir_id(user, &keys.join("/"))?;
        let assoc = find_assoc(self.conn(), dir_id, &file_name)?;

        diesel::delete(files::table.filter(files::id.eq(assoc.child_id))).execute(self.conn())?;

        Ok(())
    }
}

fn find_assoc(conn: &db::Conn, parent_id: i64, file_name: &str) -> Result<mdl::FileAssoc> {
    use crate::schema::file_assocs::dsl::*;

    file_assocs
        .filter(dir_id.eq(parent_id))
        .filter(child_name.eq(file_name))
        .first(conn)
        .map_err(Into::into)
}
