use juniper::{graphql_object, EmptySubscription, RootNode};

use crate::{
    bank::schema::{BankMutation, BankQuery},
    db::DbPool,
    nordnet::graphql_schema::NordnetQuery,
};

/*
 * Context declaration
 */
pub struct Context {
    pub dbpool: DbPool,
}

impl juniper::Context for Context {}

/*
 * Query root
 */
pub struct Query {}

#[graphql_object(context = Context)]
impl Query {
    fn bank(&self) -> BankQuery {
        BankQuery
    }

    fn nordnet(&self) -> NordnetQuery {
        NordnetQuery
    }
}

/*
 * Mutation root
 */
pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    fn bank(&self) -> BankMutation {
        BankMutation
    }
}

pub type Schema = RootNode<'static, Query, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, MutationRoot {}, EmptySubscription::new())
}
