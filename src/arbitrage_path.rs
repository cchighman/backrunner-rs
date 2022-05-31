use std::sync::Arc;

use ethabi::Token;
use ethers::prelude::*;
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
        reserve0: u128,
        decimal0: i32,
        reserve1: u128,
        decimal1: i32,
    ) -> f64 {
        return f64::powi(10.0, (decimal0 - decimal1).abs()) * reserve1 as f64 / reserve0 as f64;
    }

    pub fn arb_index(&self) -> f64 {
        self.sequence.a1().get_reserve()
            / self.sequence.b1().get_reserve()
            / self.sequence.a2().get_reserve()
            / self.sequence.b2().get_reserve()
            / self.sequence.a3().get_reserve()
            / self.sequence.b3().get_reserve()
    }

    pub fn calculate(&self) -> f64 {
        /** Optimize */
        let result = optimize_a_prime(
            self.sequence.a1().get_reserve(),
            self.sequence.b1().get_reserve(),
            self.sequence.a2().get_reserve(),
            self.sequence.b2().get_reserve(),
            self.sequence.a3().get_reserve(),
            self.sequence.b3().get_reserve(),
        );

        if !result.is_none() {
            let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();
            let method = "optimize_a_prime";

            println!(
                "Method: {} Arb Index: {:.3?} Profit: {:.2?}
                Trade {:.2?} {} for {:.2?} {:.2?} at price {:.2?}
                \t\t{} Reserves:  {:.2?} Ratio: {:.2?}  {:.2?} Reserves:  {:.2?} Ratio: {:.2?}
                Trade {:.2?} {} for {:.2?} {} at price {:.2?}
                \t\t{} Reserves:  {:.2?} Ratio: {:.2?}  {:.2?} Reserves:  {:.2?} Ratio: {}
                Trade {:.2?} {} for {:.2?} {} at price {}
                \t\t{} Reserves:  {:.2?}  Ratio: {:.2?} {:.2?} Reserves:  {:.2?} Ratio: {}",
                method,
                self.arb_index(),
                profit,
                delta_a / 10_f64.powi(self.sequence.a1().get_decimal()),
                &self.sequence.a1().get_symbol(),
                delta_b / 10_f64.powi(self.sequence.a2().get_decimal()),
                &self.sequence.b1().get_symbol(),
                (self.sequence.a1().get_reserve() / 10_f64.powi(self.sequence.a1().get_decimal()))
                    / self.sequence.b1().get_reserve()
                    / 10_f64.powi(self.sequence.b1().get_decimal()),
                self.sequence.a1().get_symbol(),
                self.sequence.a1().get_reserve(),
                self.sequence.a1().get_reserve() / self.sequence.b1().get_reserve(),
                self.sequence.b1().get_symbol(),
                self.sequence.b1().get_reserve(),
                self.sequence.b1().get_reserve() / self.sequence.a1().get_reserve(),
                delta_b / 10_f64.powi(self.sequence.a2().get_decimal()),
                self.sequence.a2().get_symbol(),
                delta_c / 10_f64.powi(self.sequence.a3().get_decimal()),
                self.sequence.a3().get_symbol(),
                (self.sequence.a2().get_reserve()
                    / 10_f64.powi(self.sequence.a2().get_decimal())
                    / (self.sequence.b2().get_reserve()
                        / 10_f64.powi(self.sequence.b2().get_decimal()))),
                self.sequence.a2().get_symbol(),
                self.sequence.a2().get_reserve(),
                self.sequence.a2().get_reserve() / self.sequence.b2().get_reserve(),
                self.sequence.b2().get_symbol(),
                self.sequence.b2().get_reserve(),
                self.sequence.b2().get_reserve() / self.sequence.a2().get_reserve(),
                delta_c / 10_f64.powi(self.sequence.a3().get_decimal()),
                self.sequence.a3().get_symbol(),
                delta_a_prime / 10_f64.powi(self.sequence.b3().get_decimal()),
                self.sequence.b3().get_symbol(),
                (self.sequence.a3().get_reserve()
                    / 10_f64.powi(self.sequence.a3().get_decimal())
                    / (self.sequence.b3().get_reserve()
                        / 10_f64.powi(self.sequence.b3().get_decimal()))),
                self.sequence.a3().get_symbol(),
                self.sequence.a3().get_reserve(),
                self.sequence.a3().get_reserve() / self.sequence.b3().get_reserve(),
                self.sequence.b3().get_symbol(),
                self.sequence.b3().get_symbol(),
                self.sequence.b3().get_reserve() / self.sequence.a3().get_reserve()
            );
        }

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
            //arb_ref.arb_index();
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
