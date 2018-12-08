use crate::auth::Auth;
use crate::mdl::File;
use crate::prelude::*;
use crate::store::Store;
use crate::svc::{self, files};
use actix_web::{Json, Path, State};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddFileForm {
    content: String,
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
    (store, auth, path, form): (
        State<impl Store<Svc = S>>,
        Auth,
        Path<String>,
        Json<AddFileForm>,
    ),
) -> Result<Json<AddResult>>
where
    S: files::Store,
{
    let svc = store.service()?;
    let (keys, name) = svc::tree::split_path(&path)?;
    let form = form.into_inner();
    let file = svc.store_file(
        &auth.user,
        files::StoreForm {
            path: keys.join("/"),
            name: name.to_string(),
            content: form.content,
        },
    )?;

    Ok(Json(AddResult {
        id: file.id,
        name: file.name,
    }))
}

pub fn make_dir<S>(
    (store, auth, path): (State<impl Store<Svc = S>>, Auth, Path<String>),
) -> Result<Json<AddResult>>
where
    S: files::MakeDir,
{
    let svc = store.service()?;
    let (keys, name) = svc::tree::split_path(&path)?;
    let file = svc.make_dir(auth.user, &keys.join("/"), name.to_string())?;
    Ok(Json(AddResult {
        id: file.id,
        name: file.name,
    }))
}

pub fn list<S>(
    (store, auth, path): (State<impl Store<Svc = S>>, Auth, Path<String>),
) -> Result<Json<ListResult>>
where
    S: files::ListFiles,
{
    let svc = store.service()?;
    let files = svc.list_files(&auth.user, &path)?;
    Ok(Json(ListResult { files }))
}

pub fn delete<S>(
    (store, auth, form): (State<impl Store<Svc = S>>, Auth, Json<files::DeleteForm>),
) -> Result<Json<()>>
where
    S: files::Delete,
{
    let svc = store.service()?;
    svc.delete(auth.user, &form)?;
    Ok(Json(()))
}
