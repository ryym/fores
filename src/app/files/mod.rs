mod add_dir;
mod list;

use self::add_dir::AddDir;
use self::list::ListFiles;
use crate::auth::Auth;
use crate::mdl::File;
use crate::prelude::*;
use crate::store::Store;
use actix_web::{Json, Path, State};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddForm {
    path: String,
    name: String,
    kind: i8, // TODO: Use FileKind
}

#[derive(Debug, Serialize)]
pub struct AddResult {
    id: i64,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct ListResult {
    files: Vec<File>,
}

pub fn add<S>(
    (store, auth, form): (State<impl Store<Svc = S>>, Auth, Json<AddForm>),
) -> Result<Json<AddResult>>
where
    S: AddDir,
{
    let svc = store.service()?;
    let file = svc.add_dir(auth.user, &form.path, form.name.clone())?;
    Ok(Json(AddResult {
        id: file.id,
        name: file.name,
    }))
}

pub fn list<S>(
    (store, auth, path): (State<impl Store<Svc = S>>, Auth, Path<String>),
) -> Result<Json<ListResult>>
where
    S: ListFiles,
{
    let svc = store.service()?;
    let files = svc.list_files(&auth.user, &path)?;
    Ok(Json(ListResult { files }))
}
