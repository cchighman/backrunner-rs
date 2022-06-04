use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::ToPrimitive;
use ethabi::Token;
use ethers::prelude::U256;
use ethers::types::transaction::eip2718::TypedTransaction;
use future::ready;
use futures::{future, StreamExt};
use futures_signals::{map_ref, signal::SignalExt};
use rayon::prelude::*;
use crate::uniswap_providers::UniswapProviders;
use crate::arb_thread_pool::spawn;
use crate::crypto_math::*;
use crate::flashbot_strategy::FlashbotStrategy;
use crate::swap_route::SwapRoute;
use crate::three_path_sequence::ThreePathSequence;
use crate::uniswap_transaction::*;
use anyhow::Result;



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

    pub async fn arb_index(&self) -> BigDecimal {
        (BigDecimal::from_str(&*self.sequence.a1().get_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*self.sequence.b1().get_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.sequence.a2().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.sequence.b2().get_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.sequence.a3().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.sequence.b3().get_reserve().to_string()).unwrap())
    }

    pub async fn calculate(sequence: ThreePathSequence) {
        let result = optimize_a_prime(
            BigDecimal::from_str(&*sequence.a1().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b1().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.a2().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b2().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.a3().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b3().get_reserve().to_string()).unwrap(),
        );

        if result.is_none() {
            ready(());
        }
        let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();
        let method = "optimize_a_prime";
        println!(
            "Method: {}  Profit: {:.3?}
                Trade {:.2?} {} for {:.2?} {} at price {:.3?}
                \t\t{} Reserves:  {} Ratio: {:.2?}  {} Reserves:  {:.3?} Ratio: {:.3?}
                Trade {:.2?} {} for {:.2?} {} at price {:.3?}
                \t\t{} Reserves:  {:.3?} Ratio: {:.2?}  {} Reserves:  {:.3?} Ratio: {:.3?}
                Trade {:.2?} {} for {:.2?} {} at price {:.3?}
                \t\t{} Reserves:  {:.3?}  Ratio: {:.2?} {} Reserves:  {:.3?} Ratio: {:.3?}",
            method,
            profit.to_f64().unwrap(),
            delta_a.to_f64().unwrap(),
            &sequence.a1().get_symbol(),
            delta_b.to_f64().unwrap(),
            &sequence.b1().get_symbol(),
            (BigDecimal::from_str(&*sequence.a1().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.b1().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap(),
            sequence.a1().get_symbol(),
            sequence.a1().get_reserve(),
            (BigDecimal::from_str(&*sequence.a1().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.b1().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap(),
            sequence.b1().get_symbol(),
            sequence.b1().get_reserve(),
            (BigDecimal::from_str(&*sequence.b1().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.a1().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap(),
            delta_b.to_f64().unwrap(),
            sequence.a2().get_symbol(),
            delta_c.to_f64().unwrap(),
            sequence.b2().get_symbol(),
            ((BigDecimal::from_str(&*sequence.a2().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(
                    &*10_i128.pow(sequence.a2().get_decimal() as u32).to_string()
                )
                .unwrap())
                / (BigDecimal::from_str(&*sequence.b2().get_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(
                        &*10_i128.pow(sequence.b2().get_decimal() as u32).to_string()
                    )
                    .unwrap()))
            .to_f64()
            .unwrap(),
            sequence.a2().get_symbol(),
            sequence.a2().get_reserve(),
            (BigDecimal::from_str(&*sequence.a2().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.b2().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap(),
            sequence.b2().get_symbol(),
            sequence.b2().get_reserve(),
            (BigDecimal::from_str(&*sequence.b2().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.a2().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap(),
            delta_c.to_f64().unwrap(),
            sequence.a3().get_symbol(),
            delta_a_prime.to_f64().unwrap(),
            sequence.b3().get_symbol(),
            (BigDecimal::from_str(&*sequence.a3().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(
                    &*10_i128.pow(sequence.a3().get_decimal() as u32).to_string()
                )
                .unwrap()
                / (BigDecimal::from_str(&*sequence.b3().get_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(
                        &*10_i128.pow(sequence.b3().get_decimal() as u32).to_string()
                    )
                    .unwrap()))
            .to_f64()
            .unwrap(),
            sequence.a3().get_symbol(),
            sequence.a3().get_reserve(),
            (BigDecimal::from_str(&*sequence.a3().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.b3().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap(),
            sequence.b3().get_symbol(),
            sequence.b3().get_reserve(),
            (BigDecimal::from_str(&*sequence.b3().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*sequence.a3().get_reserve().to_string()).unwrap())
            .to_f64()
            .unwrap()
        );

        let (source_amt, dest_amt) = ArbitragePath::dec_to_u256(
            &delta_a.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.a1().get_decimal() as u32)).unwrap(),
            ),
            &delta_b.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.b1().get_decimal() as u32)).unwrap(),
            ),
        )
        .await;

        let trade1 = SwapRoute::new(
            (
                sequence.a1().get_id().clone(),
                sequence.b1().get_id().clone(),
            ),
            source_amt.clone(),
            dest_amt,
            sequence.a1().token.pair.router.clone(),
        );

        let (source_amt, dest_amt) = ArbitragePath::dec_to_u256(
            &delta_b.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.a2().get_decimal() as u32)).unwrap(),
            ),
            &delta_c.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.b2().get_decimal() as u32)).unwrap(),
            ),
        )
        .await;

        let trade2 = SwapRoute::new(
            (
                sequence.a2().get_id().clone(),
                sequence.b2().get_id().clone(),
            ),
            source_amt,
            dest_amt,
            sequence.a2().token.pair.router.clone(),
        );

        let (source_amt, dest_amt) = ArbitragePath::dec_to_u256(
            &delta_c.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.a3().get_decimal() as u32)).unwrap(),
            ),
            &delta_a_prime.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.b3().get_decimal() as u32)).unwrap(),
            ),
        )
        .await;

        let trade3 = SwapRoute::new(
            (
                sequence.a3().get_id().clone(),
                sequence.b3().get_id().clone(),
            ),
            source_amt,
            dest_amt,
            sequence.a3().token.pair.router.clone(),
        );

        let trade_vec = vec![trade1, trade2, trade3];
        let (source_amt, dest_amt) = ArbitragePath::dec_to_u256(
            &delta_a.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.a1().get_decimal() as u32)).unwrap(),
            ),
            &delta_b.clone().mul(
                BigDecimal::from_i128(10_i128.pow(sequence.b1().get_decimal() as u32)).unwrap(),
            ),
        )
        .await;

        let flash_tx : TypedTransaction = flash_swap_v2(
            sequence.a1().token.pair_id().clone(),
            source_amt,
            dest_amt,
            SwapRoute::route_calldata(trade_vec).await,
        ).await.unwrap();

        let result = FlashbotStrategy::do_flashbot_mainnet(flash_tx).await.unwrap();
        dbg!(result);
        
    }

    pub async fn dec_to_u256(delta_a: &BigDecimal, delta_b: &BigDecimal) -> (U256, U256) {
        (
            U256::from_dec_str(&*delta_a.to_string().split_once(".").unwrap().0).unwrap(),
            U256::from_dec_str(&*delta_b.to_string().split_once(".").unwrap().0).unwrap(),
        )
    }

    //noinspection RsTypeCheck

    pub async fn init(&self, arb_ref: Arc<ArbitragePath>) {
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
             (BigDecimal::from_str(&a1.to_string()).unwrap()
            / BigDecimal::from_str(&*b1.to_string()).unwrap())
            * (BigDecimal::from_str(&*a2.to_string()).unwrap()
                / BigDecimal::from_str(&*b2.to_string()).unwrap())
            * (BigDecimal::from_str(&*a3.to_string()).unwrap()
                / BigDecimal::from_str(&*b3.to_string()).unwrap())
        };

        let future = t.for_each(move |v| {
            println!(
                "Arb Index -- path: {} Index: {:.3?}",
                arb_ref.path(),
                v.to_f64().unwrap()
            );

            if v > BigDecimal::from_f64(1.05).unwrap() {
                spawn(ArbitragePath::calculate(arb_ref.sequence.clone()))
            };
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
