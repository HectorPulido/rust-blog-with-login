use chrono::NaiveDateTime;
use pwhash::bcrypt;
use super::super::schema::users;
use serde::{Deserialize, Serialize};
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub is_admin: &'a bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewUserHandler {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginHandler {
    pub username: String,
    pub password: String,
}

impl User{
    pub fn validate_password(&self, password: &String) -> bool {
        return bcrypt::verify(password, self.password.as_str());
    }
    
    pub fn generate_password(password: &String) -> String {
        return bcrypt::hash(password).unwrap();
    }
    
    pub fn create_user<'a>(
        conn: &PgConnection,
        user: &mut NewUserHandler,
    ) -> Result<User, diesel::result::Error> {

        let hashed_password = User::generate_password(&user.password);
    
        let user = NewUser {
            username: &user.username,
            email: &user.email,
            password: &hashed_password,
            is_admin: &user.is_admin,
        };
    
        diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)
    }
}



