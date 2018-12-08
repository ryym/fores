use crate::mdl::{self, File, FileKind, User};
use crate::{db, prelude::*};
use serde_derive::Deserialize;

register_service!(AddFile);

#[derive(Debug, Deserialize)]
pub struct NewFile {
    dir_id: i64,
    name: String,
}

pub trait AddFile: db::HaveConn {
    fn add_file(&self, user: &User, form: NewFile) -> Result<File> {
        use crate::schema::{file_owners, files};
        use diesel::prelude::*;

        let conn = self.conn();

        let dir = files::table
            .select(files::table::all_columns())
            .inner_join(file_owners::table)
            .filter(files::id.eq(form.dir_id))
            .filter(file_owners::owner_id.eq(user.id))
            .first::<File>(conn)?;

        if dir.kind != FileKind::Dir {
            return Err(ErrorKind::Misc("not dir".to_owned()).into());
        }

        conn.transaction(|| {
            let file = db::files::insert(
                conn,
                user.id,
                &mdl::NewFile {
                    kind: FileKind::File,
                    name: form.name,
                },
            )?;

            db::files::associate(
                conn,
                &mdl::NewFileAssoc {
                    dir_id: dir.id,
                    child_id: file.id,
                    child_name: file.name.clone(),
                },
            )?;

            Ok(file)
        })
    }
}
