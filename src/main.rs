#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let db_url = dotenv::var("DATABASE_URL").expect("DB is not found. Shutdown");
    rocket::ignite().mount("/", routes![index]).launch();
}
