mod add_dir;

use self::add_dir::AddDir;
use crate::auth::Auth;
use crate::prelude::*;
use crate::store::Store;
use actix_web::{Json, State};

#[derive(Debug, Deserialize)]
pub struct AddForm {
    path: String,
    name: String,
    kind: i8,
}

#[derive(Debug, Serialize)]
pub struct AddResult {
    id: i64,
    name: String,
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
