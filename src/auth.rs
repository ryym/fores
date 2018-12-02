use crate::db;
use crate::mdl::User;
use crate::prelude::*;
use crate::store::Store;
use actix_web::{FromRequest, HttpRequest};

#[derive(Debug)]
pub struct Auth {
    pub user: User,
}

impl<S, Svc> FromRequest<S> for Auth
where
    S: Store<Svc = Svc>,
    Svc: db::HaveConn,
{
    type Config = ();
    type Result = Result<Self>;

    // TODO: Implement actual authentication.
    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        use crate::schema::users;
        use diesel::prelude::*;

        let svc = req.state().service()?;
        let user = users::table.order(users::id).first(svc.conn())?;
        Ok(Auth { user })
    }
}
