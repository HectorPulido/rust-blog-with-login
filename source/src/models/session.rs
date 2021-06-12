use std::{collections::HashMap, pin::Pin, sync::RwLock};
use serde::{Deserialize, Serialize};
use actix_identity::Identity;
use futures::Future;
use super::user::*;

use actix_web::{
    dev::Payload, error::ErrorUnauthorized, web, Error, FromRequest,
    HttpRequest
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sessions {
    pub map: HashMap<String, User>,
}

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);
        let sessions: Option<&web::Data<RwLock<Sessions>>> = req.app_data();
        if sessions.is_none() {
            return Box::pin(async { Err(ErrorUnauthorized("unauthorized")) });
        }
        let sessions = sessions.unwrap().clone();
        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                if let Some(user) = sessions
                    .read()
                    .unwrap()
                    .map
                    .get(&identity)
                    .map(|x| x.clone())
                {
                    return Ok(user);
                }
            };

            Err(ErrorUnauthorized("unauthorized"))
        })
    
    }
}