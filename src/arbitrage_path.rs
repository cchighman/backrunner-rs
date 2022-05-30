use crate::arb_thread_pool::spawn;
use crate::call_julia::route_cfmms;
use crate::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;
use crate::crypto_math::*;
use crate::crypto_pair::CryptoPair;
use crate::swap_route::route_calldata;
use crate::swap_route::{SwapRoute, TO_ADDRESS};
use crate::uniswap_providers::ROPSTEN_PROVIDER;

use crate::uniswap_transaction::*;
use bigdecimal::BigDecimal;
use ethabi::Token;
use ethereum_types::{Address, U256, U512};
use ethers::prelude::Bytes;
use ethers::prelude::*;
use future::ready;

use futures::{executor, future, StreamExt};
use futures_signals::signal::{Mutable, MutableSignal};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use futures_util::Future;
use num_traits::{FromPrimitive, Pow, ToPrimitive, Zero};
use rayon::prelude::*;

use crate::{arb_thread_pool, uniswap_transaction};
use ethers::prelude::coins_bip39::English;
use std::collections::HashMap;
use std::default;
use std::ops::{Deref, Div, Mul};
use std::str::FromStr;
use std::sync::Arc;
/* Babylonian Sqrt */
use sim::utils::usize_sqrt;

use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
use crate::utils::common::{SequenceToken, ThreePathSequence};
use muldiv::MulDiv;
use num_traits::real::Real;

enum SIDE {
    Left,
    Right,
}

impl ArbitragePath {
    /* TODO - For each pair */
    pub fn new(sequence: ThreePathSequence) -> Arc<Self> {
        /*
                let mut c_to_i = HashMap::<Address, i8>::new();
                let mut i_to_c = HashMap::<i8, Address>::new();

                let mut index = 1;
                /* Populate a coin index */
                for pair in &path_pairs {
                    if !c_to_i.contains_key(&pair.left_id()) {
                        c_to_i.insert(pair.left_id().clone(), index);
                        i_to_c.insert(index, pair.left_id().clone());
                        index = index + 1;
                    }
                    if !c_to_i.contains_key(&pair.right_id()) {
                        c_to_i.insert(pair.right_id().clone(), index);
                        i_to_c.insert(index, pair.right_id().clone());
                        index = index + 1;
                    }
                }
        */
        Arc::new(Self {
            sequence: sequence, /*
                                coin_to_index: c_to_i,
                                index_to_coin: i_to_c,

                                 */
        })
    }

    pub fn path(&self) -> String {
        let mut path_str: String = Default::default();
        for token in 0..self.sequence.sequence.len() {
            path_str = path_str.to_owned() + " - " + token.symbol();
        }

        return path_str;
    }
    /*
        pub fn get_calc_vec(&self) -> Vec<String> {
            let mut calcVec: Vec<String> = Default::default();

            for pair in &self.path_pairs {
                let left_coin_index = self.coin_to_index.get(&pair.left_id()).unwrap();
                let right_coin_index = self.coin_to_index.get(&pair.right_id()).unwrap();

                let cfmm: String = pair.left_reserves().to_string()
                    + ","
                    + &*pair.right_reserves().to_string()
                    + ","
                    + "0.997"
                    + ","
                    + &*left_coin_index.to_string()
                    + ","
                    + &*right_coin_index.to_string();
                calcVec.push(cfmm);
            }
            return calcVec;
        }

        pub fn convert_calc_to_swap(&self, calcVec: Vec<String>) -> Vec<SwapRoute> {
            let mut retVec: Vec<SwapRoute> = Default::default();

            for trade in calcVec {
                let parts = trade.split(",").collect::<Vec<_>>();
                let cfmm_pair_idx = parts[0].parse::<i8>().unwrap() - 1;
                let crypto_pair = self.path_pairs.get(cfmm_pair_idx as usize).unwrap();

                let source_coin = self
                    .index_to_coin
                    .get(&(parts[1]).parse::<i8>().unwrap())
                    .unwrap();
                let source_amount = U256::from_dec_str(parts[2]).unwrap();
                let dest_coin = self
                    .index_to_coin
                    .get(&(parts[3]).parse::<i8>().unwrap())
                    .unwrap();
                let dest_amount = U256::from_dec_str(parts[4]).unwrap();

                retVec.push(SwapRoute {
                    pair: crypto_pair.clone(),
                    source_amount,
                    dest_amount,
                });
            }
            return retVec;
        }
    */
    pub fn reserves_to_amount(
        &self,
        reserve0: u128,
        decimal0: i32,
        reserve1: u128,
        decimal1: i32,
    ) -> f64 {
        return f64::powi(10.0, (decimal0 - decimal1).abs()) * reserve1 as f64 / reserve0 as f64;
    }

    pub fn arb_index(&self) -> f64 {
        /*
        /** Optimize */
        let result = optimize_a_prime(
            aa1.to_f64().unwrap(),
            bb1.to_f64().unwrap(),
            aa2.to_f64().unwrap(),
            bb2.to_f64().unwrap(),
            aa3.to_f64().unwrap(),
            bb3.to_f64().unwrap(),
        );

        if !result.is_none() {
            let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();
            let method = "optimize_a_prime";

            println!(
                "Method: {}  Arb Index: {:.3?}  Profit: {:.2?}
                Trade {:.2?} {} for {:.2?} {:.2?} at price {:.2?}
                \t\t{} Reserves:  {:.2?} Ratio: {:.2?}  {:.2?} Reserves:  {:.2?} Ratio: {:.2?}
                Trade {:.2?} {} for {:.2?} {} at price {:.2?}
                \t\t{} Reserves:  {:.2?} Ratio: {:.2?}  {:.2?} Reserves:  {:.2?} Ratio: {}
                Trade {:.2?} {} for {:.2?} {} at price {}
                \t\t{} Reserves:  {:.2?}  Ratio: {:.2?} {:.2?} Reserves:  {:.2?} Ratio: {}",
                method,
                arb_index,
                profit,
                delta_a / 10_f64.powi(a1_decimal),
                &reserve_a1_symbol,
                delta_b / 10_f64.powi(a2_decimal),
                &reserve_b1_symbol,
                (a1.to_string().parse::<f64>().unwrap() / 10_f64.powi(a1_decimal))
                    / (b1.to_string().parse::<f64>().unwrap() / 10_f64.powi(b1_decimal)),
                &reserve_a1_symbol,
                &reserve_a1,
                reserve_a1.to_string().parse::<f64>().unwrap()
                    / reserve_b1.to_string().parse::<f64>().unwrap(),
                reserve_b1_symbol,
                reserve_b1,
                reserve_b1.to_string().parse::<f64>().unwrap()
                    / reserve_a1.to_string().parse::<f64>().unwrap(),
                delta_b / 10_f64.powi(a2_decimal),
                &reserve_a2_symbol,
                delta_c / 10_f64.powi(a3_decimal),
                &reserve_a3_symbol,
                (a2.to_string().parse::<f64>().unwrap() / 10_f64.powi(a2_decimal))
                    / (b2.to_string().parse::<f64>().unwrap() / 10_f64.powi(b2_decimal)),
                &reserve_a2_symbol,
                &reserve_a2,
                reserve_a2.to_string().parse::<f64>().unwrap()
                    / reserve_b2.to_string().parse::<f64>().unwrap(),
                reserve_b2_symbol,
                reserve_b2,
                reserve_b2.to_string().parse::<f64>().unwrap()
                    / reserve_a2.to_string().parse::<f64>().unwrap(),
                delta_c / 10_f64.powi(a3_decimal),
                &reserve_a3_symbol,
                delta_a_prime / 10_f64.powi(b3_decimal),
                &reserve_b3_symbol,
                (a3.to_string().parse::<f64>().unwrap() / 10_f64.powi(a3_decimal))
                    / (b3.to_string().parse::<f64>().unwrap() / 10_f64.powi(b3_decimal)),
                &reserve_a3_symbol,
                &reserve_a3,
                reserve_a3.to_string().parse::<f64>().unwrap()
                    / reserve_b3.to_string().parse::<f64>().unwrap(),
                reserve_b3_symbol,
                reserve_b3,
                reserve_b3.to_string().parse::<f64>().unwrap()
                    / reserve_a3.to_string().parse::<f64>().unwrap()
            );
        }
         */

        return 0.0;
    }

    //noinspection RsTypeCheck

    pub async fn init(&self, arb_ref: Arc<ArbitragePath>) {
        type Output = ();
        // println!("Before Spawn");

        let value6 = self.sequence.a3().get_signal();
        let value7 = self.sequence.b3().get_signal();

        let mut t = map_ref! {
            let a1 = self.sequence.a1().get_signal(),
             let b1 =self.sequence.b1().get_signal(),
             let a2 =  self.sequence.a2().get_signal(),
             let b2 =  self.sequence.b2().get_signal(),
             let a3 = value6,
             let b3 = value7 =>
            a1/b1 / a2/b2 / a3/b3
        };

        let future = t.for_each(move |v| {
            println!("Arb Index -- path: {} Index: {}", arb_ref.path(), v);
            arb_ref.arb_index();
            /* Evaluate whether we have an arb in this path */
            /*
                             /* Calculate Amounts */
                             let swap_routes =
                                 arb_ref.convert_calc_to_swap(route_cfmms(&arb_ref.get_calc_vec()));

                             /* Setup First Swap */
                             let swap_routes = swap_routes.split_first().unwrap();
                             let firstPairContract = UniswapV2Pair::new(
                                 Address::from_str(swap_routes.0.pair.pair_id()).unwrap(),
                                 (*ROPSTEN_PROVIDER.deref()).clone(),
                             );

                             /* get calldata from Pair.swap contract */
                             let first_trade = firstPairContract.swap(
                                              swap_routes.0.source_amount,
                                 swap_routes.0.dest_amount,
                                 *TO_ADDRESS,
                                 swap_route::route_calldata(&swap_routes.1),
                             );
            */

            //execute_flashbot_strategy(&first_trade.tx).await
            ready(())
        });
        spawn(future);
    }

    /* Maintains a reference to the actual pairs */
    pub fn path_pairs(&self) -> &Vec<Arc<CryptoPair>> {
        &self.path_pairs
    }
}
#[derive(Debug, Clone)]
pub struct ArbitragePath {
    sequence: ThreePathSequence,
    /*
    coin_to_index: HashMap<Address, i8>,
    index_to_coin: HashMap<i8, Address>,

     */
}
#[test]
pub fn test_abi_encoding() {
    let tokens = vec![Token::String("test".to_string())];
    let call_data = ethers::abi::encode(&tokens);
}
