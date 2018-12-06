use super::find_dir::FindDir;
use crate::mdl::{File, User};
use crate::{db, prelude::*};
use diesel::prelude::*;

register_service!(ListFiles);

pub trait ListFiles: FindDir + db::HaveConn {
    fn list_files(&self, user: &User, path: &str) -> Result<Vec<File>> {
        use crate::schema::{file_assocs, files};

        let dir_id = self.find_dir(&user, path)?;
        files::table
            .inner_join(file_assocs::table.on(file_assocs::child_id.eq(files::id)))
            .filter(file_assocs::dir_id.eq(dir_id))
            .select(files::table::all_columns())
            .get_results(self.conn())
            .map_err(Into::into)
    }
}
