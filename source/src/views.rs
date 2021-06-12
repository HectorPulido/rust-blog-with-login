extern crate diesel;

use self::diesel::prelude::*;
use super::handlers::{NewPostPublish, PostToPublish};
use super::models::*;
use super::schema::posts::dsl::*;
use super::{create_post, DbPool};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_posts(tmpl: web::Data<tera::Tera>, pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let results = web::block(move || {
        posts
            .filter(published.eq(true))
            .limit(5)
            .order(id.desc())
            .load::<Post>(&conn)
    })
    .await;

    let response = match results {
        Ok(x) => {
            let mut ctx = tera::Context::new();
            ctx.insert("title", "Hector's blog");
            ctx.insert("posts", &x);
            let s = tmpl.render("index.html", &ctx);

            match s {
                Ok(x) => HttpResponse::Ok().content_type("text/html").body(x),
                Err(e) => {
                    println!("Template error: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            println!("Result error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    };

    return response;
}

pub async fn create_a_post(
    pool: web::Data<DbPool>,
    item: web::Json<NewPostPublish>,
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let results = web::block(move || create_post(&conn, &item)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn toggle_a_post(
    pool: web::Data<DbPool>,
    item: web::Json<PostToPublish>,
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let results = web::block(move || {
        diesel::update(posts.find(item.id))
            .set(published.eq(item.activate))
            .get_result::<Post>(&conn)
    })
    .await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_a_post(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let results =
        web::block(move || diesel::delete(posts.find(user_id.into_inner())).execute(&conn)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn specific_post(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let results = web::block(move || {
        posts
            .filter(id.eq(user_id.into_inner()))
            .limit(1)
            .load::<Post>(&conn)
    })
    .await;

    let response = match results {
        Ok(x) => {
            if x.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let mut ctx = tera::Context::new();
            ctx.insert("title", "Hector's blog");
            ctx.insert("post", &x[0]);
            let s = tmpl.render("post.html", &ctx);

            match s {
                Ok(x) => HttpResponse::Ok().content_type("text/html").body(x),
                Err(e) => {
                    println!("Template error: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            println!("Result error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    };
    return response;
}

pub async fn post_creation(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Hector's blog");
    let s = tmpl.render("create_post.html", &ctx);

    match s {
        Ok(x) => HttpResponse::Ok().content_type("text/html").body(x),
        Err(e) => {
            println!("Template error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
