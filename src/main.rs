#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::prelude::*;
use rocket_contrib::{Json, Value};
use rocket::Rocket;

mod schema;
mod models;
mod db;

use models::*;
use schema::posts::dsl::*;

//fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
//    use schema::posts;
//
//    let new_post = NewPost {
//        title,
//        body,
//    };
//    diesel::insert_into(posts::table)
//        .values(&new_post)
//        .get_result(conn)
//        .expect("Error saving new post")
//}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn find_post(conn: db::Conn, pid: i32) -> QueryResult<Post> {
    posts
        .find(pid)
        .first::<Post>(&*conn)
}

#[get("/<pid>")]
fn get_post(conn: db::Conn, pid: i32) -> Json<Value> {
    let post = find_post(conn, pid).unwrap();
    Json(json!(post))

}

fn rocket() -> Rocket {
    let pool = db::init_pool();
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index])
        .mount("/post", routes![get_post])
}

fn main() {
    rocket().launch();
}
