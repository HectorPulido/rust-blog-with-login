extern crate diesel;

use self::diesel::prelude::*;
use super::super::models::session::*;
use super::super::models::user::*;
use super::super::schema::users::dsl::*;
use super::super::DbPool;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::RwLock;

#[get("/user/")]
pub async fn account(user: User) -> impl Responder {
    web::Json(user)
}

#[post("/user/")]
pub async fn create_a_user(
    pool: web::Data<DbPool>,
    item: web::Json<NewUserHandler>,
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let mut item = item.clone();

    let results = web::block(move || User::create_user(&conn, &mut item)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/user/login/")]
pub async fn login_template(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Hector's blog");
    let s = tmpl.render("login.html", &ctx);

    match s {
        Ok(x) => HttpResponse::Ok().content_type("text/html").body(x),
        Err(e) => {
            println!("Template error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/user/login/")]
pub async fn login(
    pool: web::Data<DbPool>,
    item: web::Json<LoginHandler>,
    sessions: web::Data<RwLock<Sessions>>,
    identity: Identity,
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let item_password = item.password.clone();
    let item_username = item.username.clone();

    let results = web::block(move || {
        users
            .filter(username.eq(item_username))
            .limit(1)
            .load::<User>(&conn)
    })
    .await;

    let response = match results {
        Ok(x) => {
            if x.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let user = &x[0];
            match user.validate_password(&item_password) {
                true => {
                    let user_id = user.id.to_string();
                    identity.remember(user_id.clone());

                    sessions.write().unwrap().map.insert(user_id, user.clone());

                    HttpResponse::Ok().finish()
                }
                false => HttpResponse::NotFound().finish(),
            }
        }
        Err(e) => {
            println!("Result error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    };
    return response;
}

#[get("/user/logout/")]
pub async fn logout(sessions: web::Data<RwLock<Sessions>>, identity: Identity) -> impl Responder {
    if let Some(user_id) = identity.identity() {
        identity.forget();
        sessions.write().unwrap().map.remove(&user_id);
    }
    HttpResponse::Unauthorized().finish()
}
