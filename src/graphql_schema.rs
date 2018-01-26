use diesel;
use diesel::prelude::*;

use juniper::{Context as JuniperContext, FieldResult};
use db::Conn;

use models::{Post, NewPost};

pub struct Context {
    pub pool: Conn,
}

impl JuniperContext for Context {}

graphql_object!(Post: () |&self| {
    description: "Post",

    field id() -> i32 as "unique id" {
        self.id
    }

    field title() -> &str as "title" {
        self.title
    }

    field body() -> &str as "post's body" {
        self.body
    }

    field published() -> bool as "post's status" {
        self.published
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field posts(&executor) -> FieldResult<Vec<Post>>
        as "Get all posts in the system"
    {
        use schema::posts::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        dsl::posts::order(dsl::id)
            .load::<Post>(&*connection)
            .to_field_err()
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field create_post(&executor, title: String, body: String) -> FieldResult<Post>
    as "create new post"
    {
        use schema::posts::dsl;
        let connection = executor.context().pool.clone().get().unwrap();

        connection.transaction(|| {
            let new_post = NewPost {
                title,
                body,
            };

            diesel::insert_into(dsl::posts::table)
                .values(&new_post)
                .get_result::<Post>(&*connection)?;

            dsl::posts.order(dsl::id.desc())
                .first::<Post>(&*connection)
        }).to_field_result()
    }
});
