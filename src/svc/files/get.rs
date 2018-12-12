use crate::schema::{file_assocs, file_contents, files};
use crate::svc::tree::{split_path_opt, FindDirId};
use crate::{db, mdl, prelude::*};
use diesel::prelude::*;
use serde_derive::Serialize;

register_service!(Get);

#[derive(Debug, Serialize)]
pub struct FileDetail {
    name: String,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct FileItem {
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "body")]
pub enum FileContent {
    File(FileDetail),
    Dir(Vec<FileItem>),
}

impl FileContent {
    pub fn from_file(file: mdl::File, content: String) -> FileContent {
        let detail = FileDetail {
            name: file.name,
            content,
        };
        FileContent::File(detail)
    }
}

pub trait Get: FindDirId + db::HaveConn {
    fn get_file_content(&self, user: &mdl::User, path: &str) -> Result<FileContent> {
        let conn = self.conn();

        let (file, content) = match split_path_opt(path) {
            Some((keys, name)) => {
                let parent_id = self.find_dir_id(&user, &keys.join("/"))?;
                find_file_with_content(conn, parent_id, name)?
            }
            None => {
                let root_id = self.find_dir_id(&user, "")?;
                let file = files::table.filter(files::id.eq(root_id)).first(conn)?;
                (file, None)
            }
        };

        match file.kind {
            mdl::FileKind::File => {
                // Should we consider the case that a file does not have a content?
                let content = content.unwrap_or(String::new());
                Ok(FileContent::from_file(file, content))
            }
            mdl::FileKind::Dir => {
                let files = list_children(conn, file.id)?
                    .into_iter()
                    .map(|name| FileItem { name })
                    .collect();
                Ok(FileContent::Dir(files))
            }
        }
    }
}

fn find_file_with_content(
    conn: &db::Conn,
    parent_id: i64,
    name: &str,
) -> Result<(mdl::File, Option<String>)> {
    files::table
        .inner_join(file_assocs::table.on(file_assocs::child_id.eq(files::id)))
        .left_join(file_contents::table)
        .filter(file_assocs::dir_id.eq(parent_id))
        .filter(files::name.eq(name))
        .select((
            files::table::all_columns(),
            file_contents::content.nullable(),
        ))
        .first(conn)
        .map_err(Into::into)
}

fn list_children(conn: &db::Conn, parent_id: i64) -> Result<Vec<String>> {
    file_assocs::table
        .filter(file_assocs::dir_id.eq(parent_id))
        .select(file_assocs::child_name)
        .get_results::<String>(conn)
        .map_err(Into::into)
}
