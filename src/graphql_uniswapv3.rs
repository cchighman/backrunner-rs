use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery, Response};
use reqwest::blocking::Client;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
pub type BigInt = String;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/queries/uniswapv3_pools.graphql",
    response_derives = "Debug,Serialize,PartialEq, Clone",
    schema_path = "src/queries/uniswapv3_schema.json",
    introspect = false
)]
pub struct UniswapPools;

//noinspection ALL
pub fn get_pools(
    endpoint: &str,
) -> std::result::Result<Response<uniswap_pools::ResponseData>, reqwest::Error> {
    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .build()
        .unwrap();

    let response_body =
        post_graphql::<UniswapPools, _>(&client, endpoint, uniswap_pools::Variables);
    return response_body;
}
