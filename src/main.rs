#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]

extern crate rocket;
extern crate dotenv;
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

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
    let conn = establish_connection();
    rocket::ignite().mount("/", routes![index]).launch();
}
