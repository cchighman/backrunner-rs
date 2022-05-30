use ethereum_types::U256;

pub struct UniswapPools;

pub mod uniswap_pools {
    #![allow(dead_code)]

    pub const OPERATION_NAME: &str = "UniswapPools";
    pub const QUERY : & str = "query UniswapPools {\r\n  pools(first: 100, where: {volumeUSD_gt: \"50000\"}, orderBy: reserveUSD, orderDirection: desc) {\r\n    id\r\n    token0 {\r\n      id\r\n      symbol\r\n      name\r\n      decimals\r\n\r\n    }\r\n    token1 {\r\n      id\r\n      symbol\r\n      name\r\n      decimals\r\n    }\r\n    reserveUSD\r\n    volumeUSD\r\n    reserve0\r\n    reserve1\r\n    reserveETH\r\n    token0price\r\n    token1price\r\n\r\n  }\r\n}\r\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    pub type ID = String;

    #[derive(Serialize)]
    pub struct Variables;

    pub type BigInt = String;

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pairs: Data,
    }

    #[derive(Deserialize, Debug)]
    pub struct Data {
        pub pairs: Vec<UniswapPoolsPools>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct UniswapPoolsPools {
        pub id: ID,
        pub sqrt_price: U256,
        pub liquidity: U256,
        pub fee_tier: i32,
        pub tick: i32,
        pub dex: String,
        pub token0: UniswapPoolsPoolsToken0,
        pub token1: UniswapPoolsPoolsToken1,
    }
    #[derive(Deserialize, Debug, Clone)]
    pub struct UniswapPoolsPoolsToken0 {
        pub id: ID,
        pub symbol: String,
        pub name: String,
        pub decimals: i32,
    }
    #[derive(Deserialize, Debug, Clone)]
    pub struct UniswapPoolsPoolsToken1 {
        pub id: ID,
        pub symbol: String,
        pub name: String,
        pub decimals: i32,
    }
}
/*
impl graphql_client::GraphQLQuery for UniswapPairs {
    type Variables = uniswap_pairs::Variables;
    type ResponseData = uniswap_pairs::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: uniswap_pairs::QUERY,
            operation_name: uniswap_pairs::OPERATION_NAME,
        }
    }
}
*/
