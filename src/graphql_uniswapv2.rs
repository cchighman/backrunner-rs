use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery, Response};
use reqwest::blocking::Client;
use bigdecimal::BigDecimal;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/queries/uniswapv2_schema.graphql",
    query_path = "src/queries/uniswapv2_pairs.graphql",
    response_derives = "Debug,Serialize,PartialEq, Clone",
    introspect = false
)]
pub struct UniswapPairs;

//noinspection ALL
pub async fn get_pairs(
    endpoint: &str,
) -> std::result::Result<Response<uniswap_pairs::ResponseData>, reqwest::Error> {
    // let endpoint = "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2";

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .build()
        .unwrap();

    let response_body =
        post_graphql::<UniswapPairs, _>(&client, endpoint, uniswap_pairs::Variables);
    //   let response_body2 = (response_body.as_ref());
    // dbg!("{:#?}", response_body2);

    return response_body;
}
