use super::find_dir::FindDir;
use super::replace_dir::ReplaceDir;
use crate::mdl::FileKind;
use crate::{db, mdl, prelude::*};
use diesel::prelude::*;
use serde_derive::Deserialize;

register_service!(Delete);
register_service!(DeleteDir);
register_service!(DeleteFile);

#[derive(Debug, Deserialize)]
pub struct DeleteForm {
    path: String,
    kind: FileKind,
}

pub trait Delete: DeleteDir + DeleteFile {
    fn delete(&self, user: mdl::User, form: &DeleteForm) -> Result<()> {
        match form.kind {
            FileKind::File => self.delete_file(&user, &form.path),
            FileKind::Dir => self.delete_dir(user, &form.path),
        }
    }
}

pub trait DeleteDir: ReplaceDir + db::HaveConn {
    fn delete_dir(&self, mut user: mdl::User, path: &str) -> Result<()> {
        let (keys, dir_name) = split_path(path)?;
        let parent_path = keys.join("/");

        let dir_id = self.replace_dir(&mut user.tree, &parent_path, |parent| {
            let parent = parent.as_object_mut().unwrap();
            let obj = match parent.get(dir_name) {
                Some(obj) => obj,
                None => return Err(ErrorKind::Misc("invalid path".to_owned()).into()),
            };
            let dir_id: i64 = serde_json::from_value(obj["..id"].clone()).unwrap();
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

pub trait DeleteFile: FindDir + db::HaveConn {
    fn delete_file(&self, user: &mdl::User, path: &str) -> Result<()> {
        use crate::schema::files;

        let (keys, file_name) = split_path(path)?;
        let dir_id = self.find_dir(user, &keys.join("/"))?;
        let assoc = find_assoc(self.conn(), dir_id, &file_name)?;

        diesel::delete(files::table.filter(files::id.eq(assoc.child_id))).execute(self.conn())?;

        Ok(())
    }
}

fn split_path(path: &str) -> Result<(Vec<&str>, &str)> {
    let mut keys = path.split("/").filter(|s| s.len() > 0).collect::<Vec<_>>();
    match keys.pop() {
        Some(last) => Ok((keys, last)),
        None => Err(ErrorKind::Misc("invalid path".to_owned()).into()),
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
