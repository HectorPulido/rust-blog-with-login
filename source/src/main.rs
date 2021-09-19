pub mod models;
pub mod schema;
pub mod views;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use diesel::r2d2::Pool;
use dotenv::dotenv;
use models::session::*;
use std::env;
use std::{collections::HashMap, sync::RwLock};
use tera::Tera;
use views::post::*;
use views::user::*;

use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_manager() -> ConnectionManager<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return ConnectionManager::<PgConnection>::new(database_url);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");

    let connection = get_connection_manager();

    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");

    let sessions = web::Data::new(RwLock::new(Sessions {
        map: HashMap::new(),
    }));

    println!("Running http://{}:{}", host, port);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .app_data(sessions.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("session_cookie")
                    .secure(false),
            ))
            .data(pool.clone())
            .data(tera)
            .service(fs::Files::new("/static", "./static"))
            .route("/", web::get().to(get_posts))
            .route("/", web::post().to(create_a_post))
            .route("/post/{id}/", web::get().to(specific_post))
            .route("/post/{id}/", web::delete().to(delete_a_post))
            .route("/post-creation/", web::get().to(post_creation))
            .route("/toggle-a-post/", web::post().to(toggle_a_post))
            .route("/user/", web::get().to(account))
            .route("/user/", web::post().to(create_a_user))
            .route("/user/login/", web::get().to(login_template))
            .route("/user/login/", web::post().to(login))
            .route("/user/logout/", web::get().to(logout))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
