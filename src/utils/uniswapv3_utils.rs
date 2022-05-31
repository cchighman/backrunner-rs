use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use crate::graphql_uniswapv3;

use bigdecimal::BigDecimal;
use dashmap::DashMap;
use ethereum_types::{Address, U256};
use num_bigint::BigInt;
use std::ops::Div;

use std::str::FromStr;
use std::sync::Arc;

use crate::utils::uniswap_v3_sdk::{get_amounts_for_liquidity, get_sqrt_ratio_at_tick, tick_range};

use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;
use crate::uniswapv3_pools::uniswap_pools::{UniswapPoolsPoolsToken0, UniswapPoolsPoolsToken1};

#[derive(Clone)]
pub struct UniswapPool {
    pub id: String,
    pub sqrt_price: U256,
    pub liquidity: U256,
    pub reserve0: U256,
    pub reserve1: U256,
    pub fee_tier: U256,
    pub token0price: BigDecimal,
    pub token1price: BigDecimal,
    pub dex: String,
    pub token0: UniswapPoolsPoolsToken0,
    pub token1: UniswapPoolsPoolsToken1,
}

pub fn populate_uniswapv3_pools(pair_map: &mut DashMap<String, Vec<Arc<CryptoPair>>>) {
    let pairs =
        graphql_uniswapv3::get_pools("https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3")
            .unwrap();
    //dbg!("uniswap - {#:?}", pairs);
    uniswapv3_unpack_pairs(
        pairs,
        pair_map,
        " - univ3".to_string(),
        Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
    );
}

pub fn uniswapv3_unpack_pairs(
    pairs: graphql_client::Response<graphql_uniswapv3::uniswap_pools::ResponseData>,
    pair_map: &mut DashMap<String, Vec<Arc<CryptoPair>>>,
    dex: String,
    router: Address,
) {
    for pair in pairs.data.unwrap().pools {
        let got_tick: i32 = pair.tick.unwrap().parse::<i32>().unwrap();

        let [tick_lower, tick_upper] =
            tick_range(got_tick, pair.fee_tier.parse::<i32>().unwrap().div(5), 0);
        let sqrt_lower = get_sqrt_ratio_at_tick(tick_lower);
        let sqrt_upper = get_sqrt_ratio_at_tick(tick_upper);
        let [reserve0, reserve1] = get_amounts_for_liquidity(
            &BigInt::from_str(pair.sqrt_price.as_str()).unwrap(),
            &sqrt_lower,
            &sqrt_upper,
            U256::from_dec_str(&pair.liquidity).unwrap(),
        );

        let uni_pair = DexPool {
            token0: UniswapPairsPairsTokens {
                id: Address::from_str(&pair.token0.id).unwrap(),
                name: pair.token0.name.clone(),
                decimals: pair.token0.decimals.parse::<i32>().unwrap(),
                symbol: pair.token0.symbol.clone(),
                reserve: 0.0,
            },
            token1: UniswapPairsPairsTokens {
                id: Address::from_str(&pair.token1.id).unwrap(),
                name: pair.token1.name.clone(),
                decimals: pair.token1.decimals.parse::<i32>().unwrap(),
                symbol: pair.token1.symbol.clone(),
                reserve: 0.0,
            },
            id: Address::from_str(&pair.id).unwrap(),
            sqrt_price: U256::from_dec_str(pair.sqrt_price.clone().to_string().as_str()).unwrap(),
            liquidity: U256::from_dec_str(pair.liquidity.clone().to_string().as_str()).unwrap(),
            fee_tier: pair.fee_tier.parse::<i32>().unwrap(),
            tick: got_tick,
            dex: dex.clone(),
            router,
        };

        let pair_symbol = pair.token0.symbol + &pair.token1.symbol;
        if !pair_map.contains_key::<String>(&pair_symbol) {
            let pair_vec = vec![Arc::new(CryptoPair::new(uni_pair))];
            pair_map.insert(pair_symbol, pair_vec);
        } else {
            let mut pair_vec = pair_map.get_mut(&pair_symbol).unwrap().value().clone();
            pair_vec.push(Arc::new(CryptoPair::new(uni_pair)));
            pair_map.alter(&pair_symbol, |_, _v| pair_vec);
        }
    }
}

#[test]
fn test_uniswapv3_graphql() {
    let mut crypto_pairs: DashMap<String, Vec<_>> = DashMap::new();
    populate_uniswapv3_pools(&mut crypto_pairs);
    std::dbg!("pools - {#:?}", crypto_pairs);
}
