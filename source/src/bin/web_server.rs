extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::Pool;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = get_connection_manager();

    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(pool.clone())
            .data(tera)
            .service(fs::Files::new("/static", "./static"))
            .route("/", web::get().to(views::get_posts))
            .route("/", web::post().to(views::create_a_post))
            .route("/post/{id}/", web::get().to(views::specific_post))
            .route("/post/{id}/", web::delete().to(views::delete_a_post))
            .route("/post-creation/", web::get().to(views::post_creation))
            .route("/toggle-a-post/", web::post().to(views::toggle_a_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
