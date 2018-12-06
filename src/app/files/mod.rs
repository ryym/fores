mod add_dir;
mod add_file;
mod find_dir;
mod list;

use self::add_dir::AddDir;
use self::add_file::{AddFile, NewFile};
use self::list::ListFiles;
use crate::auth::Auth;
use crate::mdl::File;
use crate::prelude::*;
use crate::store::Store;
use actix_web::{Json, Path, State};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddDirForm {
    path: String,
    name: String,
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

pub fn add_file<S>(
    (store, auth, form): (State<impl Store<Svc = S>>, Auth, Json<NewFile>),
) -> Result<Json<AddResult>>
where
    S: AddFile,
{
    let svc = store.service()?;
    let file = svc.add_file(&auth.user, form.into_inner())?;
    Ok(Json(AddResult {
        id: file.id,
        name: file.name,
    }))
}

pub fn add_dir<S>(
    (store, auth, form): (State<impl Store<Svc = S>>, Auth, Json<AddDirForm>),
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
