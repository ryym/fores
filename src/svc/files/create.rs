use crate::svc::tree::FindDir;
use crate::{db, mdl, prelude::*};
use diesel::Connection;

register_service!(Create);

#[derive(Debug)]
pub struct CreateForm {
    pub path: String,
    pub name: String,
    pub content: String,
}

pub trait Create: FindDir + db::HaveConn {
    fn create_file(&self, user: &mdl::User, form: CreateForm) -> Result<mdl::File> {
        let dir_id = self.find_dir(user, &form.path)?;
        let conn = self.conn();

        let dir = find_dir_record(conn, user, dir_id)?;

        assert!(
            dir.kind == mdl::FileKind::Dir,
            "file id ({}) must be a directory",
            dir.id
        );

        let new_file = db::files::InsertFile {
            owner_id: user.id,
            name: form.name,
            content: form.content,
        };
        conn.transaction(|| {
            let file = db::files::insert_file(conn, new_file)?;

            let assoc = mdl::NewFileAssoc {
                dir_id,
                child_id: file.id,
                child_name: file.name.clone(),
            };
            db::files::associate(conn, &assoc)?;

            Ok(file)
        })
    }
}

fn find_dir_record(conn: &db::Conn, user: &mdl::User, dir_id: i64) -> Result<mdl::File> {
    use crate::schema::{file_owners, files};
    use diesel::prelude::*;

    files::table
        .select(files::table::all_columns())
        .inner_join(file_owners::table)
        .filter(files::id.eq(dir_id))
        .filter(file_owners::owner_id.eq(user.id))
        .first::<mdl::File>(conn)
        .map_err(Into::into)
}
