use chrono::NaiveDateTime;
use super::super::schema::posts;
use serde::{Deserialize, Serialize};
use diesel::pg::PgConnection;
use diesel::prelude::*;


#[derive(Clone, Serialize, Deserialize)]
#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub author_id: i32,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
    pub author_id: &'a i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PostToggleHandler {
    pub id: i32,
    pub published: bool,
}

impl Post {
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post<'a>(
        conn: &PgConnection,
        post: &NewPostHandler,
        author_id: &i32
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());

        let post = NewPost{
            title: &post.title,
            body: &post.body,
            slug: &slug,
            author_id: &author_id,
        };
    
        diesel::insert_into(posts::table)
            .values(post)
            .get_result(conn)
    }
}

