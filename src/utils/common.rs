use crate::arbitrage_path::ArbitragePath;
use ethers::prelude::{Address, U256};
use futures_signals::map_ref;
use futures_signals::signal::{MutableSignal, SignalExt};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::future::ready;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use crate::arb_thread_pool::spawn;
use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;

/*
pub fn dec_to_int(dec_string: &str, places: i64) -> U256 {
    let rounded = U256::from_str(dec_string).unwrap().round(places);
    //  println!("{}", rounded);
    let base: i128 = 10;
    let num_exp = BigInt::from(base.pow(places as u32));
    //println!("{}", num_exp);
    let result = rounded.mul(num_exp);
    // println!("{}", result.normalized());

    return U256::from_dec_str(result.normalized().to_string().as_str()).unwrap();
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum DIRECTION {
    Default,
    Left,
    Right,
}
