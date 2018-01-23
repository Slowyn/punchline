#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

use models::*;

fn establish_connection() -> PgConnection {
    let db_url = dotenv::var("DATABASE_URL").expect("DB is not found. Shutdown");
    PgConnection::establish(&db_url)
        .expect(&format!("Error  connecting to {}", db_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    use schema::posts::dsl::*;

    let conn = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
//    rocket::ignite().mount("/", routes![index]).launch();
}
