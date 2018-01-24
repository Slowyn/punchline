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
use schema::posts;

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

#[post("/", data = "<post>")]
fn new_post(conn: db::Conn, post: Json<NewPost>) -> QueryResult<Json<Post>> {
    let new_post_inst = post.0;

    diesel::insert_into(posts::table)
        .values(&new_post_inst)
        .get_result::<Post>(&*conn)
        .map(|p|  Json(p))
}

fn rocket() -> Rocket {
    let pool = db::init_pool();
    rocket::ignite()
        .manage(pool)
        .mount("/post", routes![get_post, new_post])
}

fn main() {
    rocket().launch();
}
