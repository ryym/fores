//! We use plural form for this module name because `move` is a reserded word in Rust.

use crate::schema::{file_assocs, files};
use crate::svc::tree::{get_dir_id, split_path, ModifyDir};
use crate::{db, mdl, prelude::*};
use diesel::prelude::*;

register_service!(Move);

#[derive(Debug)]
pub struct MoveForm<'s> {
    pub src: &'s str,
    pub dest: &'s str,
}

pub trait Move: ModifyDir + db::HaveConn {
    fn move_file(&self, mut user: mdl::User, form: MoveForm) -> Result<()> {
        let (keys, name) = split_path(form.src)?;

        // Find the parent directory.
        // If the source file is a directory, remove it from the tree.
        let (parent_id, dir) = self.modify_dir(&mut user.tree, &keys.join("/"), |parent| {
            let dir = parent.as_object_mut().unwrap().remove(name);
            Ok((get_dir_id(parent), dir))
        })?;

        let conn = self.conn();

        let file = find_file(conn, parent_id, name, &dir)?;

        // Find the destination directory. And put the source directory if necessary.
        let new_parent_id = self.modify_dir(&mut user.tree, form.dest, |obj| {
            if let Some(dir) = dir {
                obj[&file.name] = dir;
            }
            Ok(get_dir_id(obj))
        })?;

        conn.transaction(|| {
            let dessoc = db::files::Dessociate {
                parent_id,
                child_id: file.id,
            };
            db::files::dessociate(conn, &dessoc)?;
            db::files::associate(conn, new_parent_id, &file)?;

            Ok(())
        })
    }
}

fn find_file(
    conn: &db::Conn,
    parent_id: i64,
    name: &str,
    dir: &Option<serde_json::Value>,
) -> Result<mdl::File> {
    match dir {
        Some(dir) => files::table.find(get_dir_id(dir)).first::<mdl::File>(conn),
        None => files::table
            .select(files::table::all_columns())
            .inner_join(file_assocs::table.on(file_assocs::child_id.eq(files::id)))
            .filter(file_assocs::dir_id.eq(parent_id))
            .filter(files::name.eq(name))
            .first(conn),
    }
    .map_err(Into::into)
}
