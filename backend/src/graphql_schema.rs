use juniper::{EmptyMutation, EmptySubscription, GraphQLObject, RootNode};

use crate::{bank::graphql_schema::BankQuery, nordnet::graphql_schema::NordnetQuery};

#[derive(GraphQLObject)]
pub struct RootQuery {
    bank: BankQuery,
    nordnet: NordnetQuery,
}

impl RootQuery {
    fn new() -> Self {
        Self {
            bank: BankQuery,
            nordnet: NordnetQuery,
        }
    }
}

pub type Schema = RootNode<'static, RootQuery, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(
        RootQuery::new(),
        EmptyMutation::new(),
        EmptySubscription::new(),
    )
}
