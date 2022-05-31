use ethereum_types::U256;

pub struct UniswapPairs;
pub mod uniswap_pairs {
    #![allow(dead_code)]

    use ethereum_types::Address;
    use serde::{Deserialize, Serialize};

    use super::*;

    pub const OPERATION_NAME: &str = "UniswapPairs";
    pub const QUERY : & str = "query UniswapPairs {\r\n  pairs(first: 100, where: {reserveUSD_gt: \"1000000\", volumeUSD_gt: \"50000\"}, orderBy: reserveUSD, orderDirection: desc) {\r\n    id\r\n    token0 {\r\n      id\r\n      symbol\r\n      name\r\n      decimals\r\n\r\n    }\r\n    token1 {\r\n      id\r\n      symbol\r\n      name\r\n      decimals\r\n    }\r\n    reserveUSD\r\n    volumeUSD\r\n    reserve0\r\n    reserve1\r\n    reserveETH\r\n    token0price\r\n    token1price\r\n\r\n  }\r\n}\r\n" ;

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

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pairs: Data,
    }

    #[derive(Deserialize, Debug)]
    pub struct Data {
        pub pairs: Vec<UniswapPairsPairs>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct UniswapPairsPairs {
        pub id: Address,
        pub token0: UniswapPairsPairsTokens,
        pub token1: UniswapPairsPairsTokens,
        pub reserve0: U256,
        pub reserve1: U256,
        pub dex: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct UniswapPairsPairsToken0 {
        pub id: Address,
        pub symbol: String,
        pub name: String,
        pub decimals: i32,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct UniswapPairsPairsToken1 {
        pub id: Address,
        pub symbol: String,
        pub name: String,
        pub decimals: i32,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct UniswapPairsPairsTokens {
        pub id: Address,
        pub symbol: String,
        pub name: String,
        pub decimals: i32,
        pub reserve: f64,
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
