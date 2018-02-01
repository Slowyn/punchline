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
#[macro_use] extern crate juniper;
extern crate juniper_rocket;

use diesel::prelude::*;
use rocket_contrib::{Json, Value};
use rocket::{Rocket, State};
use rocket::response::content;
use juniper::{EmptyMutation, RootNode};

mod schema;
mod models;
mod db;
mod graphql_schema;

use models::*;
use schema::posts::dsl::*;
use graphql_schema::Context;

type Schema = RootNode<'static, graphql_schema::QueryRoot, graphql_schema::MutationRoot>;

fn find_post(conn: db::Conn, pid: i32) -> QueryResult<Post> {
    posts
        .find(pid)
        .first::<Post>(&*conn)
}

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn rocket() -> Rocket {
    let pool = db::init_pool();
    let context = Context { pool };
    rocket::ignite()
        .manage(context)
        .manage(Schema::new(graphql_schema::QueryRoot, graphql_schema::MutationRoot))
        .mount("/", routes![graphiql, get_graphql_handler, post_graphql_handler])
}

fn main() {
    rocket().launch();
}
