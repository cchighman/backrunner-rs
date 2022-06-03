use bigdecimal::BigDecimal;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use ethabi::Token;

use crate::swap_route::{route_calldata, SwapRoute};
use ethers::prelude::{Address, U256};
use future::ready;
use futures::{future, StreamExt};
use futures_signals::{map_ref, signal::SignalExt};
use num_traits::real::Real;
use rayon::prelude::*;

use crate::arb_thread_pool::spawn;
use crate::crypto_math::*;

use crate::uniswap_transaction::*;
use crate::utils::common::ThreePathSequence;

/* Babylonian Sqrt */
impl ArbitragePath {
    /* TODO - For each pair */
    pub fn new(sequence: ThreePathSequence) -> Arc<Self> {
        Arc::new(Self { sequence: sequence })
    }

    pub fn path(&self) -> String {
        let mut path_str: String = Default::default();
        for token in 0..self.sequence.sequence.len() {
            path_str = path_str.to_owned() + " - " + self.sequence.sequence[token].get_symbol();
        }

        return path_str;
    }

    pub fn reserves_to_amount(
        &self,
        reserve0: U256,
        decimal0: i32,
        reserve1: U256,
        decimal1: i32,
    ) -> BigDecimal {
        return BigDecimal::from_f64(
            f64::powi(10.0, (decimal0 - decimal1).abs()) * reserve1 as f64 / reserve0 as f64,
        )
        .unwrap();
    }

    pub fn arb_index(&self) -> BigDecimal {
        (BigDecimal::from_str(&*self.sequence.a1().get_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*self.sequence.b1().get_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.sequence.a2().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.sequence.b2().get_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.sequence.a3().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.sequence.b3().get_reserve().to_string()).unwrap())
    }

    pub fn calculate(&self) {
        let result = optimize_a_prime(
            BigDecimal::from_str(&*self.sequence.a1().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*self.sequence.b1().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*self.sequence.a2().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*self.sequence.b2().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*self.sequence.a3().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*self.sequence.b3().get_reserve().to_string()).unwrap(),
        );

        let ten = BigDecimal::from(10);

        if !result.is_none() {
            let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();
            let method = "optimize_a_prime";

            println!(
                "Method: {} Arb Index: {} Profit: {}
                Trade {:.2?} {} for {:.2?} {} at price {}
                \t\t{} Reserves:  {} Ratio: {:.2?}  {} Reserves:  {} Ratio: {}
                Trade {:.2?} {} for {:.2?} {} at price {}
                \t\t{} Reserves:  {} Ratio: {:.2?}  {} Reserves:  {} Ratio: {}
                Trade {:.2?} {} for {:.2?} {} at price {}
                \t\t{} Reserves:  {}  Ratio: {:.2?} {} Reserves:  {} Ratio: {}",
                method,
                self.arb_index(),
                profit,
                delta_a,
                &self.sequence.a1().get_symbol(),
                delta_b,
                &self.sequence.b1().get_symbol(),
                (self.sequence.a1().get_reserve() / self.sequence.b1().get_reserve()),
                self.sequence.a1().get_symbol(),
                self.sequence.a1().get_reserve(),
                self.sequence.a1().get_reserve() / self.sequence.b1().get_reserve(),
                self.sequence.b1().get_symbol(),
                self.sequence.b1().get_reserve(),
                self.sequence.b1().get_reserve() / self.sequence.a1().get_reserve(),
                delta_b,
                self.sequence.a2().get_symbol(),
                delta_c,
                self.sequence.a3().get_symbol(),
                (BigDecimal::from_str(&*self.sequence.a2().get_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(
                        &*10_i32
                            .pow(self.sequence.a2().get_decimal() as u32)
                            .to_string()
                    )
                    .unwrap())
                    / (BigDecimal::from_str(&*self.sequence.b2().get_reserve().to_string())
                        .unwrap()
                        / BigDecimal::from_str(
                            &*10_i32
                                .pow(self.sequence.b2().get_decimal() as u32)
                                .to_string()
                        )
                        .unwrap()),
                self.sequence.a2().get_symbol(),
                self.sequence.a2().get_reserve(),
                self.sequence.a2().get_reserve() / self.sequence.b2().get_reserve(),
                self.sequence.b2().get_symbol(),
                self.sequence.b2().get_reserve(),
                self.sequence.b2().get_reserve() / self.sequence.a2().get_reserve(),
                delta_c,
                self.sequence.a3().get_symbol(),
                delta_a_prime,
                self.sequence.b3().get_symbol(),
                (BigDecimal::from_str(&*self.sequence.a3().get_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(
                        &*10_i32
                            .pow(self.sequence.a3().get_decimal() as u32)
                            .to_string()
                    )
                    .unwrap())
                    / (BigDecimal::from_str(&*self.sequence.b3().get_reserve().to_string())
                        .unwrap()
                        / BigDecimal::from_str(
                            &*10_i32
                                .pow(self.sequence.b3().get_decimal() as u32)
                                .to_string()
                        )
                        .unwrap()),
                self.sequence.a3().get_symbol(),
                self.sequence.a3().get_reserve(),
                self.sequence.a3().get_reserve() / self.sequence.b3().get_reserve(),
                self.sequence.b3().get_symbol(),
                self.sequence.b3().get_reserve(),
                self.sequence.b3().get_reserve() / self.sequence.a3().get_reserve()
            );

            let (source_amt, dest_amt) = self.dec_to_u256(
                &delta_a.clone().mul(BigDecimal::from(
                    10_i32.pow(self.sequence.a1().get_decimal() as u32),
                )),
                &delta_b.clone().mul(BigDecimal::from(
                    10_u32.pow(self.sequence.a1().get_decimal() as u32),
                )),
            );

            let trade1 = SwapRoute::new(
                (
                    self.sequence.a1().get_id().clone(),
                    self.sequence.b1().get_id().clone(),
                ),
                source_amt.clone(),
                dest_amt,
                self.sequence.a1().router().clone(),
            );

            let (source_amt, dest_amt) = self.dec_to_u256(
                &delta_a.clone().mul(BigDecimal::from(
                    10_i32.pow(self.sequence.a1().get_decimal() as u32),
                )),
                &delta_b.clone().mul(BigDecimal::from(
                    10_u32.pow(self.sequence.a1().get_decimal() as u32),
                )),
            );

            let trade2 = SwapRoute::new(
                (
                    self.sequence.a2().get_id().clone(),
                    self.sequence.b2().get_id().clone(),
                ),
                source_amt,
                dest_amt,
                self.sequence.a1().router().clone(),
            );

            let (source_amt, dest_amt) = self.dec_to_u256(
                &delta_c.clone().mul(BigDecimal::from(
                    10_i32.pow(self.sequence.a3().get_decimal() as u32),
                )),
                &delta_a_prime.clone().mul(BigDecimal::from(
                    10_u32.pow(self.sequence.b3().get_decimal() as u32),
                )),
            );
            let trade3 = SwapRoute::new(
                (
                    self.sequence.a3().get_id().clone(),
                    self.sequence.b3().get_id().clone(),
                ),
                source_amt,
                dest_amt,
                self.sequence.a1().router().clone(),
            );
            let trade_vec = vec![trade1, trade2, trade3];
            let (source_amt, dest_amt) = self.dec_to_u256(
                &delta_a.clone().mul(BigDecimal::from(
                    10_i32.pow(self.sequence.a1().get_decimal() as u32),
                )),
                &delta_b.clone().mul(BigDecimal::from(
                    10_u32.pow(self.sequence.b1().get_decimal() as u32),
                )),
            );
            /*
            let flash_tx = flash_swap_v2(
                self.sequence.a1().token.pair_id().clone(),
                source_amt,
                dest_amt,
                route_calldata(trade_vec),
            );
            */
        }
    }

    pub fn dec_to_u256(&self, delta_a: &BigDecimal, delta_b: &BigDecimal) -> (U256, U256) {
        (
            U256::from_dec_str(&*delta_a.to_string()).unwrap(),
            U256::from_dec_str(&*delta_a.to_string()).unwrap(),
        )
    }

    //noinspection RsTypeCheck

    pub fn init(&self, arb_ref: Arc<ArbitragePath>) {
        type Output = ();

        let value6 = self.sequence.a3().get_signal();
        let value7 = self.sequence.b3().get_signal();

        let t = map_ref! {
            let a1 = self.sequence.a1().get_signal(),
             let b1 =self.sequence.b1().get_signal(),
             let a2 =  self.sequence.a2().get_signal(),
             let b2 =  self.sequence.b2().get_signal(),
             let a3 = value6,
             let b3 = value7 =>
            a1/b1 * a2/b2 * a3/b3
        };

        let future = t.for_each(move |v| {
            println!("Arb Index -- path: {} Index: {}", arb_ref.path(), v);

            arb_ref.calculate();

            //execute_flashbot_strategy(&first_trade.tx).await

            ready(())
        });
        spawn(future);
    }
}

#[derive(Debug, Clone)]
pub struct ArbitragePath {
    sequence: ThreePathSequence,
}
#[test]
pub fn test_abi_encoding() {
    let tokens = vec![Token::String("test".to_string())];
    let call_data = ethers::abi::encode(&tokens);
}
