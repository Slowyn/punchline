use diesel;
use diesel::prelude::*;

use juniper::{Context as JuniperContext, FieldResult};
use db::Pool;

use models::{Post, NewPost};

pub struct Context {
    pub pool: Pool,
}

impl JuniperContext for Context {}

graphql_object!(Post: () |&self| {
    description: "Post",

    field id() -> i32 as "unique id" {
        self.id
    }

    field title() -> &String as "title" {
        &self.title
    }

    field body() -> &String as "post's body" {
        &self.body
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

        Ok(dsl::posts
            .order(dsl::id)
            .load(&*connection)
            .unwrap())
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field create_post(&executor, title: String, body: String) -> FieldResult<Post>
    as "create new post"
    {
        use schema::posts::dsl;
        let connection = executor.context().pool.clone().get().unwrap();
            let new_post = NewPost {
                title,
                body,
            };

            diesel::insert_into(::schema::posts::table)
                .values(&new_post)
                .get_result::<Post>(&*connection);

            Ok(dsl::posts
                .order(dsl::id.desc())
                .first::<Post>(&*connection)
                .unwrap())
    }
});
