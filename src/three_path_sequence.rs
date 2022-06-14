use std::collections::HashMap;
use std::future::ready;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::prelude::Address;
use ethers::prelude::U256;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use ethers::types::transaction::eip2718::TypedTransaction;
use itertools::Itertools;
use crate::arb_thread_pool::spawn;
use crate::crypto_math::optimize_a_prime;
use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use crate::path_sequence::PathSequence;
use crate::sequence_token::SequenceToken;
use crate::swap_route::SwapRoute;
use crate::three_path_sequence;
use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;
use crate::utils::common::DIRECTION;
use super::uniswap_providers::*;
use crate::uniswap_transaction::*;
use crate::flashbot_strategy::utils::*;
use std::any::Any;


#[derive(Debug, Clone)]
pub struct ThreePathSequence {
    pub(crate) sequence: Vec<SequenceToken>,
    pub(crate) pairs: Vec<Arc<CryptoPair>>
}

pub async fn cyclic_order(
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: &HashMap<Address, Arc<CryptoPair>>
)->Result<Arc<(dyn Any + 'static + Sync + Send)>, anyhow::Error> {
        let a1_b3 = crypto_path[0].left_symbol() == crypto_path[2].right_symbol();
        let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
        let b2_a3 = crypto_path[1].right_symbol() == crypto_path[2].left_symbol();
        let a1_a2 = crypto_path[0].left_symbol() == crypto_path[1].left_symbol();
        let b1_b3 = crypto_path[0].right_symbol() == crypto_path[2].right_symbol();
        let b1_b2 = crypto_path[0].right_symbol() == crypto_path[1].right_symbol();
        let a2_a3 = crypto_path[1].left_symbol() == crypto_path[2].left_symbol();
        let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();
        let b1_a3 = crypto_path[0].right_symbol() == crypto_path[2].left_symbol();
        let b2_b3 = crypto_path[1].right_symbol() == crypto_path[2].right_symbol();
        let a1_a3 = crypto_path[0].left_symbol() == crypto_path[2].left_symbol();
        let a2_b3 = crypto_path[1].left_symbol() == crypto_path[2].right_symbol();

        let scenario_1 = a1_b3 && b1_a2 && b2_a3;
        let scenario_2 = a1_a2 && b1_b3 && b2_a3;
        let scenario_3 = a1_b3 && b1_b2 && a2_a3;
        let scenario_4 = a1_b2 && b1_b3 && a2_a3;
        let scenario_5 = a1_a2 && b1_a3 && b2_b3;
        let scenario_6 = a1_a3 && b1_a2 && b2_b3;
        let scenario_7 = a1_a3 && b1_b2 && a2_b3;
        let scenario_8 = a1_b2 && b1_a3 && a2_b3;

            let pair_id_1 = crypto_path[0].pair_id();

            let pair_1 = crypto_pairs.get_key_value(pair_id_1).unwrap().1;

            let pair_id_2 = crypto_path[1].pair_id();
            let pair_2 = crypto_pairs.get_key_value(pair_id_2).unwrap().1;

            let pair_id_3 = crypto_path[2].pair_id();
            let pair_3 = crypto_pairs.get_key_value(pair_id_3).unwrap().1;

            let token_a1 = if scenario_2 || scenario_4 || scenario_5 || scenario_8 {
                Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Right))
            } else {
                Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Left))
            };

            let token_b1 = if scenario_1 || scenario_3 || scenario_6 || scenario_7 {
                Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Right))
            } else {
                Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Left))
            };

            let token_a2 = if scenario_1 || scenario_2 || scenario_5 || scenario_6 {
                Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Left))
            } else {
                Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Right))
            };

            let token_b2 = if scenario_3 || scenario_4 || scenario_7 || scenario_8 {
                Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Left))
            } else {
                Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Right))
            };

            let token_a3 = if scenario_1 || scenario_2 || scenario_3 || scenario_4 {
                Option::from(SequenceToken::new(pair_3.clone(), DIRECTION::Left))
            } else {
                Option::from(SequenceToken::new(pair_3.clone(), DIRECTION::Right))
            };

            let token_b3 = if scenario_5 || scenario_6 || scenario_7 || scenario_8 {
                Option::from(SequenceToken::new(pair_3.clone(), DIRECTION::Left))
            } else {
                Option::from(SequenceToken::new(pair_3.clone(), DIRECTION::Right))
            };
            Ok(ThreePathSequence::new(vec![pair_1.clone(), pair_2.clone(), pair_3.clone()], vec![
                token_a1.unwrap(),
                token_b1.unwrap(),
                token_a2.unwrap(),
                token_b2.unwrap(),
                token_a3.unwrap(),
                token_b3.unwrap(),
            ]).await)
   }

pub  fn is_arbitrage_pair(crypto_path: &Vec<CryptoPair>) ->bool{
    let a1_b3 = crypto_path[0].left_symbol() == crypto_path[2].right_symbol();
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    let b2_a3 = crypto_path[1].right_symbol() == crypto_path[2].left_symbol();
    let a1_a2 = crypto_path[0].left_symbol() == crypto_path[1].left_symbol();
    let b1_b3 = crypto_path[0].right_symbol() == crypto_path[2].right_symbol();
    let b1_b2 = crypto_path[0].right_symbol() == crypto_path[1].right_symbol();
    let a2_a3 = crypto_path[1].left_symbol() == crypto_path[2].left_symbol();
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();
    let b1_a3 = crypto_path[0].right_symbol() == crypto_path[2].left_symbol();
    let b2_b3 = crypto_path[1].right_symbol() == crypto_path[2].right_symbol();
    let a1_a3 = crypto_path[0].left_symbol() == crypto_path[2].left_symbol();
    let a2_b3 = crypto_path[1].left_symbol() == crypto_path[2].right_symbol();

    let scenario_1 = a1_b3 && b1_a2 && b2_a3;
    let scenario_2 = a1_a2 && b1_b3 && b2_a3;
    let scenario_3 = a1_b3 && b1_b2 && a2_a3;
    let scenario_4 = a1_b2 && b1_b3 && a2_a3;
    let scenario_5 = a1_a2 && b1_a3 && b2_b3;
    let scenario_6 = a1_a3 && b1_a2 && b2_b3;
    let scenario_7 = a1_a3 && b1_b2 && a2_b3;
    let scenario_8 = a1_b2 && b1_a3 && a2_b3;

    if scenario_1
        || scenario_2
        || scenario_3
        || scenario_4
        || scenario_5
        || scenario_6
        || scenario_7
        || scenario_8
    {
       true
    } else {
    false
    }
}

/* 
pub async fn test_is_arbitrage_pair_true()->Result<bool, anyhow::Error> {
    let pair1 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from(1000000_u64),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F000").unwrap(),
            symbol: "USDT".to_string(),
            name: "USDT".to_string(),
            decimals: 18,
            reserve: U256::from(1000000_u64),
        },
    });
    let pair2 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665FFFF").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from(1000000_u64),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F111").unwrap(),
            symbol: "DAI".to_string(),
            name: "DAI".to_string(),
            decimals: 18,
            reserve: U256::from(1000000_u64),
        },
    });
    let pair3 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F222").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F000").unwrap(),
            symbol: "USDT".to_string(),
            name: "USDT".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F111").unwrap(),
            symbol: "DAI".to_string(),
            name: "DAI".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });

    let arb_vec = vec![pair1, pair2, pair3];
    println!("Result: {}", is_arbitrage_pair(&arb_vec));
    assert!(is_arbitrage_pair(&arb_vec));
    Ok(())
}
*/

impl ThreePathSequence {
    pub fn a3(&self) -> &SequenceToken {
        &self.sequence[4]
    }
    pub fn b3(&self) -> &SequenceToken {
        &self.sequence[5]
    }

    pub fn path(&self) -> String {
        let mut path_str: String = Default::default();
        for token in 0..self.sequence.len() {
            path_str = path_str.to_owned() + " - " + self.sequence[token].get_symbol();
        }
        path_str
    }

    pub async fn calculate(sequence: Arc<ThreePathSequence>) {
        let result = optimize_a_prime(
            BigDecimal::from_str(&*sequence.a1().get_pending_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b1().get_pending_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.a2().get_pending_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b2().get_pending_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.a3().get_pending_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b3().get_pending_reserve().to_string()).unwrap(),
        );

        if !result.is_none() {
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
                (BigDecimal::from_str(&*sequence.a1().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.b1().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap(),
                sequence.a1().get_symbol(),
                sequence.a1().get_pending_reserve(),
                (BigDecimal::from_str(&*sequence.a1().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.b1().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap(),
                sequence.b1().get_symbol(),
                sequence.b1().get_pending_reserve(),
                (BigDecimal::from_str(&*sequence.b1().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.a1().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap(),
                delta_b.to_f64().unwrap(),
                sequence.a2().get_symbol(),
                delta_c.to_f64().unwrap(),
                sequence.b2().get_symbol(),
                ((BigDecimal::from_str(&*sequence.a2().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(
                        &*10_i128.pow(sequence.a2().get_decimal() as u32).to_string()
                    )
                    .unwrap())
                    / (BigDecimal::from_str(&*sequence.b2().get_pending_reserve().to_string()).unwrap()
                        / BigDecimal::from_str(
                            &*10_i128.pow(sequence.b2().get_decimal() as u32).to_string()
                        )
                        .unwrap()))
                .to_f64()
                .unwrap(),
                sequence.a2().get_symbol(),
                sequence.a2().get_pending_reserve(),
                (BigDecimal::from_str(&*sequence.a2().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.b2().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap(),
                sequence.b2().get_symbol(),
                sequence.b2().get_pending_reserve(),
                (BigDecimal::from_str(&*sequence.b2().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.a2().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap(),
                delta_c.to_f64().unwrap(),
                sequence.a3().get_symbol(),
                delta_a_prime.to_f64().unwrap(),
                sequence.b3().get_symbol(),
                (BigDecimal::from_str(&*sequence.a3().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(
                        &*10_i128.pow(sequence.a3().get_decimal() as u32).to_string()
                    )
                    .unwrap()
                    / (BigDecimal::from_str(&*sequence.b3().get_pending_reserve().to_string()).unwrap()
                        / BigDecimal::from_str(
                            &*10_i128.pow(sequence.b3().get_decimal() as u32).to_string()
                        )
                        .unwrap()))
                .to_f64()
                .unwrap(),
                sequence.a3().get_symbol(),
                sequence.a3().get_pending_reserve(),
                (BigDecimal::from_str(&*sequence.a3().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.b3().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap(),
                sequence.b3().get_symbol(),
                sequence.b3().get_pending_reserve(),
                (BigDecimal::from_str(&*sequence.b3().get_pending_reserve().to_string()).unwrap()
                    / BigDecimal::from_str(&*sequence.a3().get_pending_reserve().to_string()).unwrap())
                .to_f64()
                .unwrap()
            );

            let (source_amt, dest_amt) = ThreePathSequence::dec_to_u256(
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

            let (source_amt, dest_amt) = ThreePathSequence::dec_to_u256(
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

            let (source_amt, dest_amt) = ThreePathSequence::dec_to_u256(
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
            let (source_amt, dest_amt) = ThreePathSequence::dec_to_u256(
                &delta_a.clone().mul(
                    BigDecimal::from_i128(10_i128.pow(sequence.a1().get_decimal() as u32)).unwrap(),
                ),
                &delta_b.clone().mul(
                    BigDecimal::from_i128(10_i128.pow(sequence.b1().get_decimal() as u32)).unwrap(),
                ),
            )
            .await;
               
            let flash_tx: TypedTransaction = flash_swap_v2(
                sequence.a1().token.pair_id().clone(),
                source_amt,
                dest_amt,
                SwapRoute::route_calldata(trade_vec).await.unwrap()
            ).await.unwrap();

           println!("Flash Tx: {}", &flash_tx.data().unwrap());

            // Acquire pending tx
            let mut pending_tx = sequence.pending_txs();
            pending_tx.push(flash_tx);

            let bundle_result = send_flashswap_bundle(pending_tx).await;
            if bundle_result.as_ref().is_err() {
                println!("Flash bundle could not be submitted.  Reason: {:#}", bundle_result.as_ref().err().unwrap());
            } 
        } 
    }

    pub fn pending_txs(&self)->Vec<TypedTransaction> {
        let txs: Vec<TypedTransaction> = Default::default();

        let pair_tx: Vec<TypedTransaction> = self.pairs.iter().map(|pair| pair.get_pending_txs()).flatten().collect_vec();
        pair_tx
    }


    pub async fn dec_to_u256(delta_a: &BigDecimal, delta_b: &BigDecimal) -> (U256, U256) {
        (
            U256::from_dec_str(&*delta_a.to_string().split_once(".").unwrap().0).unwrap(),
            U256::from_dec_str(&*delta_b.to_string().split_once(".").unwrap().0).unwrap(),
        )
    }

   
}

#[async_trait]
impl PathSequence for ThreePathSequence {
    async fn new(pairs: Vec<Arc<CryptoPair>>,path: Vec<SequenceToken>)->Arc<(dyn Any + 'static + Sync + Send)> {
        let instance = Arc::new(Self{pairs, sequence: path});
        instance.init(instance.clone()).await;
        instance
    }       
  
    fn a1(&self) -> &SequenceToken {
        &self.sequence[0]
    }
    fn b1(&self) -> &SequenceToken {
        &self.sequence[1]
    }

    fn a2(&self) -> &SequenceToken {
        &self.sequence[2]
    }
    fn b2(&self) -> &SequenceToken {
        &self.sequence[3]
    }

    fn as_any(&self)->&dyn Any {
        self
    }

    fn arb_index(&self)-> BigDecimal{
        (BigDecimal::from_str(&*self.a1().get_pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*self.b1().get_pending_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.a2().get_pending_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.b2().get_pending_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.a3().get_pending_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.b3().get_pending_reserve().to_string()).unwrap())
    }

     async fn init(&self, arb_ref: Arc<(dyn Any + 'static + Sync + Send)>)->Result<(),anyhow::Error> {
        let pending_a3_signal = self.a3().get_pending_signal();
        let pending_b3_signal = self.b3().get_pending_signal();
        
        let confirmed_a3_signal = self.a3().get_confirmed_signal();
        let confirmed_b3_signal = self.b3().get_confirmed_signal();
        
        let pending_seq = arb_ref.downcast_ref::<ThreePathSequence>().unwrap().clone();
        let confirmed_seq = arb_ref.downcast_ref::<ThreePathSequence>().unwrap().clone();

        let pending_update = map_ref! {
            let a1 = self.a1().get_pending_signal(),
             let b1 =self.b1().get_pending_signal(),
             let a2 =  self.a2().get_pending_signal(),
             let b2 =  self.b2().get_pending_signal(),
             let a3 = pending_a3_signal,
             let b3 = pending_b3_signal =>
             (BigDecimal::from_str(&a1.to_string()).unwrap()
            / BigDecimal::from_str(&*b1.to_string()).unwrap())
            * (BigDecimal::from_str(&*a2.to_string()).unwrap()
                / BigDecimal::from_str(&*b2.to_string()).unwrap())
            * (BigDecimal::from_str(&*a3.to_string()).unwrap()
                / BigDecimal::from_str(&*b3.to_string()).unwrap())
        };

        let confirmed_update = map_ref! {
            let a1 = self.a1().get_confirmed_signal(),
             let b1 =self.b1().get_confirmed_signal(),
             let a2 =  self.a2().get_confirmed_signal(),
             let b2 =  self.b2().get_confirmed_signal(),
             let a3 = confirmed_a3_signal,
             let b3 = confirmed_b3_signal =>
             (BigDecimal::from_str(&a1.to_string()).unwrap()
            / BigDecimal::from_str(&*b1.to_string()).unwrap())
            * (BigDecimal::from_str(&*a2.to_string()).unwrap()
                / BigDecimal::from_str(&*b2.to_string()).unwrap())
            * (BigDecimal::from_str(&*a3.to_string()).unwrap()
                / BigDecimal::from_str(&*b3.to_string()).unwrap())
        };


        let pending_future = pending_update.for_each(move |v| {
            println!(
                "Pending Tx - Arb Index -- path: {} Index: {:.3?}",
                pending_seq.clone().path().clone(),
                v.to_f64().unwrap()
            );

            if v > BigDecimal::from_f64(1.05).unwrap() {
                spawn(ThreePathSequence::calculate(Arc::new(pending_seq.clone())));
            }
            ready(())
        });

        let confirmed_future = confirmed_update.for_each(move |v| {
            println!(
                "Confirmed Tx - Arb Index -- path: {} Index: {:.3?}",
                confirmed_seq.clone().path().clone(),
                v.to_f64().unwrap()
            );

            if v > BigDecimal::from_f64(1.05).unwrap() {
                spawn(ThreePathSequence::calculate(Arc::new(confirmed_seq.clone())));
            }
            ready(())
        });
        spawn(pending_future);
        spawn(confirmed_future);
        
        Ok(())
    }

}

/* 
#[test]
pub fn test_is_arbitrage_pair_false() {
    let pair1 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F000").unwrap(),
            symbol: "USDT".to_string(),
            name: "USDT".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });
    let pair2 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665FFFF").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F111").unwrap(),
            symbol: "DAI".to_string(),
            name: "DAI".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });
    let pair3 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F222").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F000").unwrap(),
            symbol: "USDT".to_string(),
            name: "USDT".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F555").unwrap(),
            symbol: "WBTC".to_string(),
            name: "WBTC".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });

    let arb_vec = vec![pair1, pair2, pair3];
    println!("Result: {}", three_path_sequence::is_arbitrage_pair(&arb_vec));
    assert!(!three_path_sequence::is_arbitrage_pair(&arb_vec));
}
*/

#[test]
pub fn test_cyclic_order() {
    let pair1 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F000").unwrap(),
            symbol: "USDT".to_string(),
            name: "USDT".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });
    let pair2 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665FFFF").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F111").unwrap(),
            symbol: "DAI".to_string(),
            name: "DAI".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });
    let pair3 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F222").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc22").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F000").unwrap(),
            symbol: "USDT".to_string(),
            name: "USDT".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665F111").unwrap(),
            symbol: "DAI".to_string(),
            name: "DAI".to_string(),
            decimals: 18,
            reserve: U256::from(1000000),
        },
    });

    let mut crypto_pairs: HashMap<Address, Arc<CryptoPair>> = HashMap::new();
    let mut crypto_paths = vec![pair1.clone(), pair2.clone(), pair3.clone()];

    crypto_pairs.insert(pair1.pair_id().clone(), Arc::new(pair1.clone()));
    crypto_pairs.insert(pair2.pair_id().clone(), Arc::new(pair2.clone()));
    crypto_pairs.insert(pair3.pair_id().clone(), Arc::new(pair3.clone()));

    let ordered = three_path_sequence::cyclic_order(crypto_paths, &crypto_pairs).unwrap();
   // let arb_path = ArbitragePath::new(ordered.clone());

    println!(
        "a1: {}, b1: {}, a2: {}, b2: {}, a3: {}, b3: {}",
        ordered.a1().get_symbol(),
        ordered.b1().get_symbol(),
        ordered.a2().get_symbol(),
        ordered.b2().get_symbol(),
        ordered.a3().get_symbol(),
        ordered.b3().get_symbol()
    );
    assert!(ordered.a1().get_symbol() == ordered.b3().get_symbol());
    assert!(ordered.b1().get_symbol() == ordered.a2().get_symbol());
    assert!(ordered.b2().get_symbol() == ordered.a3().get_symbol());

   // arb_path.calculate();
}


