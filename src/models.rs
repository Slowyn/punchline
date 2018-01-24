#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use schema::posts;

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub  struct NewPost {
    pub title: String,
    pub body: String,
}