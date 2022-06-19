use std::collections::HashMap;
use std::str::FromStr;

use ethers::prelude::{Address, U256};

use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use crate::graphql_uniswapv2;
use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;

pub async fn uniswapv2_unpack_pairs(
    pairs: graphql_client::Response<graphql_uniswapv2::uniswap_pairs::ResponseData>,
    pair_map: &mut HashMap<Address, CryptoPair>,
    dex: String,
    router: Address,
) {
    for pair in pairs.data.unwrap().pairs {
        let uni_pair = DexPool {
            token0: UniswapPairsPairsTokens {
                id: Address::from_str(&*pair.token0.id.clone()).unwrap(),
                name: pair.token0.name.clone(),
                decimals: pair.token0.decimals.parse::<i32>().unwrap(),
                symbol: pair.token0.symbol.clone(),
                reserve: U256::from_dec_str(&*pair.reserve0.round(0).to_string()).unwrap(),
            },
            token1: UniswapPairsPairsTokens {
                id: Address::from_str(&*pair.token1.id.clone()).unwrap(),
                name: pair.token1.name.clone(),
                decimals: pair.token1.decimals.parse::<i32>().unwrap(),
                symbol: pair.token1.symbol.clone(),
                reserve: U256::from_dec_str(&*pair.reserve1.round(0).to_string()).unwrap(),
            },
            id: Address::from_str(&pair.id).unwrap(),
            sqrt_price: Default::default(),
            liquidity: Default::default(),
            tick: Default::default(),
            dex: dex.clone(),
            router: router,
            fee_tier: Default::default(),
        };

        if !pair_map.contains_key::<Address>(&uni_pair.id) {
            let pair = CryptoPair::new(uni_pair.clone());
            pair_map.insert(uni_pair.id.clone(), pair);
        } else {
            let pair = pair_map.get_key_value(&uni_pair.id).unwrap();
            pair_map.insert(uni_pair.id.clone(), CryptoPair::new(uni_pair));
        }
    }
}

pub async fn populate_uniswapv2_pairs(pair_map: &mut HashMap<Address, CryptoPair>) {
    let pairs =
        graphql_uniswapv2::pairs("https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2")
            .await
            .unwrap();
    //dbg!("uniswap - {#:?}", &pairs);
    uniswapv2_unpack_pairs(
        pairs,
        pair_map,
        " - univ2".to_string(),
        Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap(),
    )
    .await;
}

pub async fn populate_sushiswap_pairs(pair_map: &mut HashMap<Address, CryptoPair>) {
    let pairs =
        graphql_uniswapv2::pairs("https://api.thegraph.com/subgraphs/name/sushiswap/exchange")
            .await
            .unwrap();
    //dbg!("sushi - {#:?}", pairs);
    uniswapv2_unpack_pairs(
        pairs,
        pair_map,
        " - sushi".to_string(),
        Address::from_str("0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F").unwrap(),
    )
    .await;
}

