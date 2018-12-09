use crate::auth::Auth;
use crate::mdl::File;
use crate::prelude::*;
use crate::store::Store;
use crate::svc::{self, files};
use actix_web::{Json, Path, State};
use serde_derive::{Deserialize, Serialize};

// Create

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum CreateForm {
    Dir,
    File { content: String },
}

#[derive(Debug, Serialize)]
pub struct CreateResult {
    id: i64,
    name: String,
}

pub fn create<S>(
    (store, auth, path, form): (
        State<impl Store<Svc = S>>,
        Auth,
        Path<String>,
        Json<CreateForm>,
    ),
) -> Result<Json<CreateResult>>
where
    S: files::MakeDir + files::Create,
{
    let svc = store.service()?;
    let (keys, name) = svc::tree::split_path(&path)?;
    let path = keys.join("/");

    let file = match form.into_inner() {
        CreateForm::Dir => svc.make_dir(auth.user, &path, name.to_string())?,
        CreateForm::File { content } => svc.create_file(
            &auth.user,
            files::CreateForm {
                path,
                name: name.to_string(),
                content,
            },
        )?,
    };

    Ok(Json(CreateResult {
        id: file.id,
        name: file.name,
    }))
}

// List

#[derive(Debug, Serialize)]
pub struct ListResult {
    files: Vec<File>,
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

// Delete

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum DeleteForm {
    Dir,
    File,
}

pub fn delete<S>(
    (store, auth, path, form): (
        State<impl Store<Svc = S>>,
        Auth,
        Path<String>,
        Json<DeleteForm>,
    ),
) -> Result<Json<()>>
where
    S: files::DeleteFile + files::DeleteDir,
{
    let svc = store.service()?;
    match form.into_inner() {
        DeleteForm::File => svc.delete_file(&auth.user, &path),
        DeleteForm::Dir => svc.delete_dir(auth.user, &path),
    }?;
    Ok(Json(()))
}
