extern crate diesel;

use self::diesel::prelude::*;
use super::super::models::post::*;
use super::super::models::user::*;
use super::super::schema::posts::dsl::*;
use super::super::DbPool;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_posts(tmpl: web::Data<tera::Tera>, pool: web::Data<DbPool>, user: Option<User>) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let is_logged = match user {
        Some(_) => true,
        None => false
    };

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
            ctx.insert("logged", &is_logged);
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
    item: web::Json<NewPostHandler>,
    user: User
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    if !user.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let results = web::block(move || Post::create_post(&conn, &item, &user.id)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn toggle_a_post(
    pool: web::Data<DbPool>,
    item: web::Json<PostToggleHandler>,
    user: User
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    if !user.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let results = web::block(move || {
        diesel::update(posts.find(item.id))
            .set(published.eq(item.published))
            .get_result::<Post>(&conn)
    })
    .await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_a_post(
    pool: web::Data<DbPool>, 
    post_id: web::Path<i32>, 
    user: User
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    if !user.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let results =
        web::block(move || diesel::delete(posts.find(post_id.into_inner())).execute(&conn)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn specific_post(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
    user: Option<User>
) -> impl Responder {
    let conn = pool.get().expect("could not get db connection");

    let is_logged = match user {
        Some(_) => true,
        None => false
    };

    let is_admin = match user {
        Some(user) => user.is_admin,
        None => false
    };

    let results = web::block(move || {
        posts
            .filter(id.eq(post_id.into_inner()))
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
            ctx.insert("logged", &is_logged);
            ctx.insert("is_admin", &is_admin);
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

pub async fn post_creation(tmpl: web::Data<tera::Tera>, user: User) -> impl Responder {

    if !user.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

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
