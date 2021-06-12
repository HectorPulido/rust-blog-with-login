pub mod handlers;
pub mod models;
pub mod schema;
pub mod views;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::handlers::NewPostPublish;
use self::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}

pub fn get_connection_manager() -> ConnectionManager<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return ConnectionManager::<PgConnection>::new(database_url);
}

pub fn create_post<'a>(
    conn: &PgConnection,
    post: &NewPostPublish,
) -> Result<Post, diesel::result::Error> {
    use schema::posts;

    let new_post = NewPost {
        title: &post.title,
        body: &post.body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}
