use juniper::GraphQLObject;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::nordnet::client;

#[derive(GraphQLObject)]
struct NordnetAccount {
    /// The account identifier (id) of the account. The id is unique within the session. Not applicable for partners
    accid: Option<i32>, // TODO: use i64
    /// The Nordnet account number
    accno: i32, // TODO: use i64
    /// Account alias can be set on Nordnet by the end user
    alias: String,
    /// Account type identifier
    atyid: Option<i32>,
    /// Description to why the account is blocked. The language specified in the request is used in this reply so it can be displayed to the end user
    blocked_reason: Option<String>,
    /// True if this is the default account
    default: bool,
    /// True if the account is blocked. No queries can be made to this account
    is_blocked: Option<bool>,
    /// Translated account type.
    account_type: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn nordnetAccounts() -> FieldResult<Vec<NordnetAccount>> {
        Ok(vec![NordnetAccount {
            accid: Some(1),
            accno: 1,
            alias: "".to_string(),
            atyid: Some(1),
            blocked_reason: Some("".to_string()),
            default: true,
            is_blocked: Some(false),
            account_type: "".to_string(),
        }])
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
