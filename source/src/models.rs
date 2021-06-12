use super::schema::posts;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Post", 4)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("body", &self.body)?;
        s.serialize_field("published", &self.published)?;
        s.end()
    }
}
