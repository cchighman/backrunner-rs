use std::collections::HashMap;
use std::future::ready;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::str::FromStr;
use std::sync::Arc;
use std::vec;

use super::uniswap_providers::*;
use crate::arb_thread_pool::spawn;
use crate::contracts::bindings::ierc20::IERC20;
use crate::crypto_math::{optimize_a_prime, optimize_a_prime_2};
use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use crate::flashbot_strategy::utils::*;
use crate::path_sequence::PathSequence;
use crate::sequence_token::SequenceToken;
use crate::swap_route::SwapRoute;
use crate::three_path_sequence;
use crate::uniswap_transaction::*;
use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;
use crate::utils::common::DIRECTION;
use crate::utils::u256_decimal::format_units;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::prelude::Address;
use ethers::prelude::U256;
use ethers::types::transaction::eip2718::TypedTransaction;
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use itertools::Itertools;
use num_bigint::BigInt;
use num_rational::Ratio;
use std::any::Any;
use crate::cfmmrouter::optimal_route;


#[derive(Debug, Clone)]
pub struct ThreePathSequence {
    pub(crate) seq_id: u8,
    pub(crate) sequence: Vec<SequenceToken>,
    pub(crate) pairs: Vec<Arc<CryptoPair>>,
}

pub enum SCENARIO {
    DEFAULT,
    WETH,
    DIFFERENT_COIN,
}

pub fn left_cyclic_symbol(symbol: &str) -> &str {
    symbol
}

pub fn right_cyclic_symbol(symbol: &str) -> &str {
    symbol
}

pub fn evaluate_symbols(left_symbol: &str, right_symbol: &str, context: Option<SCENARIO>) -> bool {
    if context.is_none() {
        return left_symbol.eq(right_symbol);
    }

    let scenario = context.unwrap();
    /* Enables extended scenarios for three path with ending scenarios in any coin. */
    if matches!(scenario, SCENARIO::DIFFERENT_COIN) {
        return true;
    }
    false
}

pub async fn cyclic_order(
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: &HashMap<Address, Arc<CryptoPair>>,
) -> Result<Arc<(dyn Any + 'static + Sync + Send)>, anyhow::Error> {
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
    /* Specific Three Coin Triangular Arbitrage Scenarios */

    let mut seq_id: u8 = 0;

    // USDT - USDC - USDC - WETH - WETH - USDT
    let scenario_1 = a1_b3 && b1_a2 && b2_a3;
    if scenario_1 {
        seq_id = 1;
    }

    //DAI - WETH - DAI - USDT - USDT - WETH
    let scenario_2 = a1_a2 && b1_b3 && b2_a3;
    if scenario_2 {
        seq_id = 2;
    }

    //DAI - WETH - USDT - WETH - USDT - DAI
    let scenario_3 = a1_b3 && b1_b2 && a2_a3;
    if scenario_3 {
        seq_id = 3;
    }

    //DAI - USDT - WETH - DAI - WETH - USDT
    let scenario_4 = a1_b2 && b1_b3 && a2_a3;
    if scenario_4 {
        seq_id = 4;
    }

    // B3 is left
    //DAI - USDT - DAI - WETH - USDT - WETH
    let scenario_5 = a1_a2 && b1_a3 && b2_b3;
    if scenario_5 {
        seq_id = 5;
    }

    //DAI - USDT - USDT - WETH - DAI - WETH
    let scenario_6 = a1_a3 && b1_a2 && b2_b3;
    if scenario_6 {
        seq_id = 6;
    }

    //DAI - USDT - WETH - USDT - DAI - WETH
    let scenario_7 = a1_a3 && b1_b2 && a2_b3;
    if scenario_7 {
        seq_id = 7;
    }

    //DAI - USDT - WETH - DAI - USDT - WETH
    let scenario_8 = a1_b2 && b1_a3 && a2_b3; //ambiguous
                                              // a1_right - b1_left - a2_right - b2_left - a3_right_b3_left
                                              // b1-a1-b2-a2-b3-a3
                                              //
    if scenario_8 {
        seq_id = 8;
    }


    /*
    Method: optimize_a_prime_2  Profit: 20186830 Seq: 8 Path:  - USDC - DAI - DAI - SPOOL - WETH - USDC
                    Trade 869826 USDC for 860261 DAI amount_in: 857706 Exchange Rate: 1 USDC=0.999148 DAI
                    Trade 860261 DAI for 828598 SPOOL amount_in: 215786 amount_out: 210834 Exchange Rate: 1 DAI=2.442961 SPOOL
                    Trade 828598 WETH for 21056656 USDC amount_in: 826111 amount_out: 20993445 Exchange Rate: 1 WETH=1201.236430 USDC
    */
    // b2_a3 a1_a3, b2_b3, a1_a3
    let pair_id_1 = crypto_path[0].pair_id();

    let pair_1 = crypto_pairs.get_key_value(pair_id_1).unwrap().1;

    let pair_id_2 = crypto_path[1].pair_id();
    let pair_2 = crypto_pairs.get_key_value(pair_id_2).unwrap().1;

    let pair_id_3 = crypto_path[2].pair_id();
    let pair_3 = crypto_pairs.get_key_value(pair_id_3).unwrap().1;

    /* **Fuzzy Matching** - Final Validations */
    // Connectors must match.
    if token_b1.clone().unwrap().symbol() == token_a2.clone().unwrap().symbol()
        && token_b2.clone().unwrap().symbol() == token_a3.clone().unwrap().symbol() && seq_id > 0
    {
        if token_a1.clone().unwrap().symbol() ==  "WETH" && token_b1.clone().unwrap().symbol() == "WETH" {
            println!("seq_id: {} - a1: {} - {:?} - b1: {} - {:?} - a2: {} - {:?} - b2: {} - {:?} - a3: {} - {:?} - b3: {} - {:?}", seq_id, token_a1.clone().unwrap().symbol(),token_a1.clone().unwrap().direction(), token_b1.clone().unwrap().symbol(),token_b1.clone().unwrap().direction(),token_a2.clone().unwrap().symbol(),token_a2.clone().unwrap().direction(),token_b2.clone().unwrap().symbol(),token_b2.clone().unwrap().direction(),token_a3.clone().unwrap().symbol(),token_a3.clone().unwrap().direction(), token_b3.clone().unwrap().symbol(),token_b3.clone().unwrap().direction());
        }
    
        Ok(ThreePathSequence::new(
            seq_id,
            vec![pair_1.clone(), pair_2.clone(), pair_3.clone()],
            vec![
                token_a1.unwrap(),
                token_b1.unwrap(),
                token_a2.unwrap(),
                token_b2.unwrap(),
                token_a3.unwrap(),
                token_b3.unwrap(),
            ],
        )
        .await)
    } else {
        Err(anyhow::format_err!("Unsupported Sequence"))
    }
}

pub fn is_arbitrage_pair(crypto_path: &Vec<CryptoPair>) -> bool {
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
        return true;
    }
    return false;

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
            path_str = path_str.to_owned() + self.sequence[token].symbol();
            if token < self.sequence.len() - 1 {
                path_str += " - ";
            }
        }
        path_str
    }

    pub fn seq_id(&self) -> u8 {
        self.seq_id
    }

    pub async fn calculate(sequence: Arc<ThreePathSequence>, pending: bool) {
       
        let a1 = &sequence.a1().pending_reserve();
        let b1 = &sequence.b1().pending_reserve();
        let a2 = &sequence.a2().pending_reserve();
        let b2 = &sequence.b2().pending_reserve();
        let a3 = &sequence.a3().pending_reserve();
        let b3 = &sequence.b3().pending_reserve();

        let result = optimize_a_prime_2(&a1, &b1, &a2, &b2, &a3, &b3);

        if result.is_some() {
            let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();

            if profit > U256::zero() {
                let one = U256::from(1_i8);

                let delta_a_rat = delta_a.to_big_rational();
                let delta_b_rat = delta_b.to_big_rational();
                let delta_c_rat = delta_c.to_big_rational();
                let delta_a_prime_rat = delta_a_prime.to_big_rational();

                let delta_a_dec = ratio_to_dec(delta_a_rat);
                let delta_b_dec = ratio_to_dec(delta_b_rat);
                let delta_c_dec = ratio_to_dec(delta_c_rat);
                let delta_a_prime_dec = ratio_to_dec(delta_a_prime_rat);

                let delta_a_amt_out = SequenceToken::get_amount_out(delta_a, *a1, *b1);
                let delta_a_amt_in = SequenceToken::get_amount_in(delta_a_amt_out.unwrap(), *a1, *b1);
                
                if !delta_a_amt_in.is_none() {
                    
                
                
                let delta_b_amt_out = SequenceToken::get_amount_out(delta_a_amt_out.unwrap(), *a2, *b2);
                let delta_b_amt_in = SequenceToken::get_amount_in(delta_b_amt_out.unwrap(), *a2, *b2);
        
                let delta_c_amt_out = SequenceToken::get_amount_out(delta_b_amt_out.unwrap(), *a3, *b3);
                let delta_c_amt_in = SequenceToken::get_amount_in(delta_c_amt_out.unwrap(), *a3, *b3);
        

                let dec = (sequence.a1().decimal() - sequence.b1().decimal()).abs();
                let trade1 = SwapRoute::new(
                    (*sequence.a1().id(), *sequence.b1().id()),
                    delta_a_amt_out.unwrap(),
                    U256::zero(),
                    U256::zero(),
                    U256::zero(),
                    sequence.a1().token.pair.router,
                    *sequence.a1().pair_id(),
                    *sequence.a2().pair_id()
                );

                let trade2 = SwapRoute::new(
                    (*sequence.a2().id(), *sequence.b2().id()),
                    delta_b_amt_in.unwrap(),
                    delta_b_amt_out.unwrap(),
                    U256::zero(),
                    U256::zero(),
                    sequence.a2().token.pair.router,
                    *sequence.a2().pair_id(),
                    *sequence.a2().pair_id()
                );

                let trade3 = SwapRoute::new(
                    (*sequence.a3().id(), *sequence.b3().id()),
                    delta_c_amt_in.unwrap(),
                    delta_c_amt_out.unwrap(),
                    U256::zero(),
                    U256::zero(),
                    sequence.a3().token.pair.router,
                    *sequence.a3().pair_id(),
                    *sequence.a2().pair_id()
                );

                /* Consider whether we will need a fouth trade to convert final token for flash repayment */
                if &sequence.a1().symbol() != &sequence.b3().symbol()
                    && !&sequence.b3().symbol().eq("WETH")
                {}

                let method = "optimize_a_prime_2";
                println!(
                    "\n\nMethod: {} Profit: {:.6} Arb Index: {:.4} Seq: {} Path: {}
                        Trade {} {} for {} {} at Exchange Rate: 1 {}={:.12} {} - Pair: {:#?}
                        \t\tId: {:#x} - {} \t Id: {:#x} - {}
                        \t\tdelta_a: {:.25}\t\t amount_out: {:.25}\t reserves: {} 
                        \t\tdelta_b: {:.25}\t\t amount_in: {:.25}\t reserves: {}
                        Trade {} {} for {} {} at Exchange Rate: 1 {}={:.12} {} - Pair: {:#?}
                        \t\tId: {:#x} - {} \t Id: {:#x} - {}
                        \t\tdelta_b: {:.25}\t\t amount_out: {:.25}\t reserves: {}
                        \t\tdelta_c: {:.25}\t\t amount_in: {:.25}\t reserves: {}
                        Trade {} {} for {} {} at Exchange Rate: 1 {}={:.12} {} - Pair: {:#?}
                        \t\tId: {:#x} - {} \t Id: {:#x} - {}
                        \t\tdelta_c: {:.25}\t\t amount_out: {:.25}\t reserves: {}
                        \t\tdelta_a_prime: {:.25}\t\t amount_in: {:.25}\t reserves: {}\n",
                    method,
                    profit,
                    sequence.arb_index(),
                    sequence.seq_id(),
                    sequence.path(),
                    // Trade 1
                    &delta_a,
                    &sequence.a1().symbol(),
                    &delta_b,
                    &sequence.b1().symbol(),
                    &sequence.a1().symbol(),
                    &sequence.a1().decimal_price().unwrap(),
                    &sequence.b1().symbol(),
                    &sequence.a1().token.pair_id(),
                    &sequence.a1().id(),
                    &sequence.a1().decimal(),
                    &sequence.b1().id(),
                    &sequence.b1().decimal(),
                    &delta_a.to_string(),
                    &delta_a_amt_out.unwrap().to_string(),
                    &sequence.a1().token.pair.token0.reserve.to_string(),
                    &delta_b.to_string(),
                    &delta_a_amt_in.unwrap(),
                    &sequence.a1().token.pair.token1.reserve.to_string(),

                    // Trade 2
                    &delta_b,
                    &sequence.a2().symbol(),
                    &delta_c,
                    &sequence.b2().symbol(),
                    &sequence.a2().symbol(),
                    &sequence.a2().decimal_price().unwrap(),
                    &sequence.b2().symbol(),
                    &sequence.a2().token.pair_id(),
                    &sequence.a2().id(),
                    &sequence.a2().decimal(),
                    &sequence.b2().id(),
                    &sequence.b2().decimal(),
                    &delta_b.to_string(),
                    &delta_b_amt_out.unwrap().to_string(),
                    &sequence.a2().token.pair.token0.reserve,
                    &delta_c.to_string(),
                    &delta_b_amt_in.unwrap().to_string(),
                    &sequence.a2().token.pair.token1.reserve,

                    // Trade 3
                    &delta_c,
                    &sequence.a3().symbol(),
                    &delta_a_prime,
                    &sequence.b3().symbol(),
                    &sequence.a3().symbol(),
                    &sequence.a3().decimal_price().unwrap(),
                    &sequence.b3().symbol(),
                    &sequence.a3().token.pair_id(),
                    &sequence.a3().id(),
                    &sequence.a3().decimal(),
                    &sequence.b3().id(),
                    &sequence.b3().decimal(),
                    &delta_c,
                    &delta_c_amt_out.unwrap(),
                    &sequence.a3().pending_reserve(),
                    &delta_a_prime,
                    &delta_c_amt_in.unwrap(),
                    &sequence.b3().pending_reserve()
                );
                // }

                let first_pair =
                    Address::from_str("0x144eC5ABF328f8d477Cd6238bAE5aa027bDDfD1E").unwrap();

                let trade_vec = vec![trade1, trade2, trade3];

                let flash_token = IERC20::new(sequence.a1().pair_id().clone(), mainnet::client.clone());
                let flash_repayment =
                    flash_token.transfer(first_pair.clone(), delta_a.mul(U256::from(997_i16)));

                let calls: Vec<
                    ethers::prelude::builders::ContractCall<
                        ethers::prelude::SignerMiddleware<
                            ethers::prelude::Provider<ethers::prelude::Http>,
                            ethers::prelude::Wallet<ethers::prelude::k256::ecdsa::SigningKey>,
                        >,
                        bool,
                    >,
                > = vec![flash_repayment];

                let tx_out = SwapRoute::route_calldata(trade_vec.clone(), calls.clone()).await.unwrap();
                println!("{:#02x}", tx_out);
                
                    let flash_tx: TypedTransaction = flash_swap_v2(
                        first_pair.clone(),
                        delta_a,
                        delta_b,
                        SwapRoute::route_calldata(trade_vec.clone(), calls.clone()).await.unwrap(),
                    )
                    .await
                    .unwrap();
                

                let mut tx_vect: Vec<TypedTransaction> = Default::default();

                // Acquire pending tx
                if pending {
                    tx_vect = sequence.pending_txs();
                }
                tx_vect.push(flash_tx);

                let flash_tx = flash_swap_v2(
                    *sequence.a1().token.pair_id(),
                    delta_a,
                    delta_b,
                    SwapRoute::route_calldata(trade_vec.clone(), calls.clone())
                        .await
                        .unwrap(),
                );

                // println!("Flash Tx: {}", &flash_tx.data().unwrap());

                

                let bundle_result = send_flashswap_bundle(tx_vect).await;
                if bundle_result.as_ref().is_err() {
                    println!(
                        "Flash bundle could not be submitted.  Reason: {:#}",
                        bundle_result.as_ref().err().unwrap()
                    );
                }
            }
        }
    }
    }

    pub fn pending_txs(&self) -> Vec<TypedTransaction> {
        let txs: Vec<TypedTransaction> = Default::default();

        let pair_tx: Vec<TypedTransaction> = self
            .pairs
            .iter()
            .flat_map(|pair| pair.pending_txs())
            .collect_vec();
        pair_tx
    }

    pub async fn dec_to_u256(delta_a: &BigDecimal, delta_b: &BigDecimal) -> (U256, U256) {
        (
            U256::from_dec_str(&*delta_a.to_string().split_once('.').unwrap().0).unwrap(),
            U256::from_dec_str(&*delta_b.to_string().split_once('.').unwrap().0).unwrap(),
        )
    }

    pub fn arb_index_2(&self) -> Ratio<BigInt> {
        let pair1 = self
            .a1()
            .pending_reserve()
            .checked_div(self.b1().pending_reserve());
        let pair2 = self
            .a2()
            .pending_reserve()
            .checked_div(self.b2().pending_reserve());
        let pair3 = self
            .a3()
            .pending_reserve()
            .checked_div(self.b3().pending_reserve());
        let is_none = pair1.is_none() || pair2.is_none() || pair3.is_none();
        if !is_none {
            return pair1
                .unwrap()
                .mul(pair2.unwrap())
                .mul(pair3.unwrap())
                .to_big_rational();
        }
        Ratio::from_float(1_0_f64).unwrap()
    }
}

#[async_trait]
impl PathSequence for ThreePathSequence {
    async fn new(
        seq_id: u8,
        pairs: Vec<Arc<CryptoPair>>,
        path: Vec<SequenceToken>,
    ) -> Arc<(dyn Any + 'static + Sync + Send)> {
        let instance = Arc::new(Self {
            seq_id,
            pairs,
            sequence: path,
        });
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn arb_index(&self) -> BigDecimal {
        (BigDecimal::from_str(&*self.a1().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*self.b1().pending_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.a2().pending_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.b2().pending_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.a3().pending_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.b3().pending_reserve().to_string()).unwrap())
    }

    async fn init(
        &self,
        arb_ref: Arc<(dyn Any + 'static + Sync + Send)>,
    ) -> Result<(), anyhow::Error> {
        let pending_a3_signal = self.a3().pending_signal();
        let pending_b3_signal = self.b3().pending_signal();

        let confirmed_a3_signal = self.a3().confirmed_signal();
        let confirmed_b3_signal = self.b3().confirmed_signal();

        let pending_seq = arb_ref.downcast_ref::<ThreePathSequence>().unwrap().clone();
        let confirmed_seq = arb_ref.downcast_ref::<ThreePathSequence>().unwrap().clone();

        let pending_update = map_ref! {
             let a1 = self.a1().pending_signal(),
             let b1 =self.b1().pending_signal(),
             let a2 =  self.a2().pending_signal(),
             let b2 =  self.b2().pending_signal(),
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
            let a1 = self.a1().confirmed_signal(),
             let b1 =self.b1().confirmed_signal(),
             let a2 =  self.a2().confirmed_signal(),
             let b2 =  self.b2().confirmed_signal(),
             let a3 = confirmed_a3_signal,
             let b3 = confirmed_b3_signal =>
             (BigDecimal::from_str(&*a1.to_string()).unwrap()
            / BigDecimal::from_str(&*b1.to_string()).unwrap())
            * (BigDecimal::from_str(&*a2.to_string()).unwrap()
                / BigDecimal::from_str(&*b2.to_string()).unwrap())
            * (BigDecimal::from_str(&*a3.to_string()).unwrap()
                / BigDecimal::from_str(&*b3.to_string()).unwrap())
        };

        let pending_future = pending_update.for_each(move |v| {
            println!(
                "Pending Tx - Arb Index -- path: {} Arb Index: {:.3?}",
                pending_seq.clone().path(),
                v.to_f64().unwrap()
            );

            if v > BigDecimal::from_f64(1.05).unwrap() {
                spawn(ThreePathSequence::calculate(
                    Arc::new(pending_seq.clone()),
                    true,
                ));
            }
            ready(())
        });

        let confirmed_future = confirmed_update.for_each(move |v| {
            
                        println!(
                            "Confirmed Tx - Arb Index -- path: {} Arb Index: {:.5} a1: {} b1: {} a2: {} b2: {} a3: {} b3:{}",
                            confirmed_seq.clone().path().clone(),
                            v.to_f64().unwrap(),
                            confirmed_seq.clone().a1().pending_reserve(),
                            confirmed_seq.clone().b1().pending_reserve(),
                            confirmed_seq.clone().a2().pending_reserve(),
                            confirmed_seq.clone().b2().pending_reserve(),
                            confirmed_seq.clone().a3().pending_reserve(),
                            confirmed_seq.clone().b3().pending_reserve()
                        );
            
            if v > BigDecimal::from_f64(1.05).unwrap() {
                spawn(ThreePathSequence::calculate(
                    Arc::new(confirmed_seq.clone()),
                    false,
                ));
            }
            ready(())
        });
        spawn(pending_future);
        spawn(confirmed_future);

        Ok(())
    }
}

use crate::utils::conversions::*;
use crate::utils::ratio_as_decimal::*;
use crate::utils::u256_decimal::*;

/*
#[test]
pub fn test_is_arbitrage_pair_false() {
    let pair1 = CryptoPair::new(DexPool {
        id: Address::from_str("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),0x
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

pub fn ratio_to_dec(value: Ratio<BigInt>) -> BigDecimal {
    let top_bytes = value.numer().to_bytes_le();
    let top = BigInt::from_bytes_le(top_bytes.0, &top_bytes.1);

    let bottom_bytes = value.denom().to_bytes_le();
    let bottom = BigInt::from_bytes_le(bottom_bytes.0, &bottom_bytes.1);
    BigDecimal::from(top) / BigDecimal::from(bottom)
}

#[tokio::test]
pub async fn test_calculate() {
    let nine_seven = U256::from(997_u16);

    let link_id: Address =
    Address::from_str("0xa36085f69e2889c224210f603d836748e7dc0088").unwrap();
let dai_id: Address =
    Address::from_str("0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa").unwrap();
let usdc_id: Address =
    Address::from_str("0xb7a4F3E9097C08dA09517b5aB877F7a917224ede").unwrap();
let weth_id: Address =
    Address::from_str("0xd0A1E359811322d97991E03f863a0C30C2cF029C").unwrap();
let uni_id: Address =
    Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
let uni2_id: Address =
    Address::from_str("0x075a36ba8846c6b6f53644fdd3bf17e5151789dc").unwrap();
let aave_id: Address =
    Address::from_str("0xB597cd8D3217ea6477232F9217fa70837ff667Af").unwrap();

let usdt_id: Address =
    Address::from_str("0x13512979ADE267AB5100878E2e0f485B568328a4").unwrap();
let wbtc_id: Address =
    Address::from_str("0xD1B98B6607330172f1D991521145A22BCe793277").unwrap();
let usdc2_id: Address =
    Address::from_str("0xc2569dd7d0fd715b054fbf16e75b001e5c0c1115").unwrap();

let aave_weth_pair: Address =
    Address::from_str("0xcc997c4593723e69df02e6ffc8937519e3d98200").unwrap();
let aave_link_pair: Address =
    Address::from_str("0x144eC5ABF328f8d477Cd6238bAE5aa027bDDfD1E").unwrap();
let dai_usdc_pair: Address =
    Address::from_str("0x5769e99b0c5dc4f8a1657d9f7c4ef7e7b015583d").unwrap();
let usdc_weth_pair: Address =
    Address::from_str("0x44892ab8F7aFfB7e1AdA4Fb956CCE2a2f3049619").unwrap();
let dai_weth_pair: Address =
    Address::from_str("0xB10cf58E08b94480fCb81d341A63295eBb2062C2").unwrap();
let link_usdc_pair: Address =
    Address::from_str("0xeb18CF20528a94882c08661D5Ee0bb8A93aC4143").unwrap();
let dai_link_pair: Address =
    Address::from_str("0x459125c711a250084a986b9fc698159865f1805e").unwrap();
let link_weth_pair: Address =
    Address::from_str("0x5ae45101eB47752Ea0068F432735cF00F6C849bD").unwrap();

let dai_uni_pair: Address =
    Address::from_str("0xFA73472326E0e0128E2CA6CeB1964fd77F4AE78d").unwrap();

let dai_uni2_pair: Address =
    Address::from_str("0x2D97c3049E6CB27eFf4d40AC0e430Ceca1080129").unwrap();

let uni2_weth_pair: Address =
    Address::from_str("0x5C58BB94b633Fdb7f03bD84781c9d8dBE3F44d37").unwrap();
let uni2_usdc_pair: Address =
    Address::from_str("0x175E6841D63c1a2A23d6d3e79ea93F33a212828D").unwrap();

let uni2_wbtc_pair = Address::from_str("0x39140e356ae52d77000697E4A15b8dFB911a4467").unwrap();
let uni2_usdc2_pair = Address::from_str("0xFe77F3cC8F3d4a1cDFeF7CED8e276aC816A42F07").unwrap();
let usdc2_wbtc_pair = Address::from_str("0x6c962C0B2c888Eb000D5Dd7d86af8B5c41575d45").unwrap();


    /*
       // Kovan - Block: 32964211


       
        Pair: 0x175E6841D63c1a2A23d6d3e79ea93F33a212828D
           uni: 0x075a36ba8846c6b6f53644fdd3bf17e5151789dc
           usdc: 0xb7a4F3E9097C08dA09517b5aB877F7a917224ede
    */
    let r1 = U256::from(400000000000000000000_u128);
    let r2 = U256::from(100000000_u128);
    
    /*
      Pair: 0x5769e99b0c5dc4f8a1657d9f7c4ef7e7b015583d-
        dai - 0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa
        usdc - 0xb7a4F3E9097C08dA09517b5aB877F7a917224ede
    */
    let r3 = U256::from(33855945375163989564_u128);
    let r4 = U256::from(1819454765041_u128);
    /*

    Pair: 0x2D97c3049E6CB27eFf4d40AC0e430Ceca1080129
       uni:0x075a36ba8846c6b6f53644fdd3bf17e5151789dc
       dai:0x4f96fe3b7a6cf9725f59d353f723c1bdb64ca6aa
    */
    let r5 = U256::from(10000000000000000000_u128);
    let r6 = U256::from(100000000000000000000_u128);

    /*
     Pair: 0xFA73472326E0e0128E2CA6CeB1964fd77F4AE78d
            uni:0x1f9840a85d5af5bf1d1762f925bdaddc4201f984
            dai:0x4f96fe3b7a6cf9725f59d353f723c1bdb64ca6aa

    */
    let r7 = U256::from(11875934361112000990_u128);
    let r8 = U256::from(49991513488463610192891_u128);
    /*

     Pair: 0x24710bfdc5ff47d461094c0d48fc63d275e26203d274e15532811a03d5e96a44
         usdt:0x13512979ade267ab5100878e2e0f485b568328a4
         usdc:0xb7a4f3e9097c08da09517b5ab877f7a917224ede

    */
    let r8 = U256::from(591641604068_u128);
    let r9 = U256::from(17063746290902230_u128);

    /*
     Pair: 0x44892ab8F7aFfB7e1AdA4Fb956CCE2a2f3049619-
         usdc - 0xb7a4F3E9097C08dA09517b5aB877F7a917224ede
         weth - 0xd0A1E359811322d97991E03f863a0C30C2cF029C

    */
    let r10 = U256::from(3747966330136191_u128);
    let r11 = U256::from(761741414892174630_u128);

    /*         Pair: 0xB10cf58E08b94480fCb81d341A63295eBb2062C2
               dai - 0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa
               weth - 0xd0a1e359811322d97991e03f863a0c30c2cf029c
    */

    let r12 = U256::from(2542694953010569227021456_u128);
    let r13 = U256::from(134572952094767475984_u128);

    /*
          Pair:0x771fbe887e04536B0a39dFa86ACdbDf1b0bD8BF6
              uni: 0x075a36ba8846c6b6f53644fdd3bf17e5151789dc
              link: 0xa36085f69e2889c224210f603d836748e7dc0088
    */

    let r14 = U256::from(25000000000000000000_u128);
    let r15 = U256::from(30402500000000000000_u128);

    /*
           Pair: 0x144eC5ABF328f8d477Cd6238bAE5aa027bDDfD1E
               link: 0xa36085f69e2889c224210f603d836748e7dc0088
               aave: 0xB597cd8D3217ea6477232F9217fa70837ff667Af
    */
    let r16 = U256::from(4000000000000000000_u128);
    let r17 = U256::from(308000000000000000_u128);

    /*
           Pair: 0x459125c711a250084a986b9fc698159865f1805e
               dai:0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa
               link:0xa36085f69e2889c224210f603d836748e7dc0088
    */
    let r18 = U256::from(4552453415191126545061_u128);
    let r19 = U256::from(2308607454100309268919_u128);

    /*
    Pair: 0x5ae45101eB47752Ea0068F432735cF00F6C849bD
        link:0xa36085f69e2889c224210f603d836748e7dc0088
        weth:0xd0A1E359811322d97991E03f863a0C30C2cF029C

      */
    let r20 = U256::from(10397578570167698679693_u128);
    let r21 = U256::from(1654220086545182421_u128);

    /*
    Pair: 0x4341737DE0Cf2746088418Fc2c50F433a38b3d54
        link:0xa36085f69e2889c224210f603d836748e7dc0088
        wbtc: 0xD1B98B6607330172f1D991521145A22BCe793277
       */
    let r22 = U256::from(6000000000000000000_u128);
    let r23 = U256::from(178260_u128);

    /*
    Pair: 0xcc997c4593723e69df02e6ffc8937519e3d98200
        aave: 0xB597cd8D3217ea6477232F9217fa70837ff667Af
        weth: 0xd0A1E359811322d97991E03f863a0C30C2cF029C
     */
    let r24 = U256::from(409100012097810849_u128);
    let r25 = U256::from(81560236_u128);

    /*
        Pair: 0x5C58BB94b633Fdb7f03bD84781c9d8dBE3F44d37
            uni: 0x075a36ba8846c6b6f53644fdd3bf17e5151789dc
            weth:  0xd0A1E359811322d97991E03f863a0C30C2cF029C
    */
    let r26 = U256::from(53431921438720603120554_u128);
    let r27 = U256::from(1121766166233_u128);

    /*
         Pair: 0xeb18CF20528a94882c08661D5Ee0bb8A93aC4143
             link:0xa36085f69e2889c224210f603d836748e7dc0088
             usdc:0xb7a4F3E9097C08dA09517b5aB877F7a917224ede

    */
    let r28 = U256::from(1000000000000000000_u128);
    let r29 = U256::from(6940000_u128);

    /*
    Pair: 0xFA73472326E0e0128E2CA6CeB1964fd77F4AE78d
        uni:0x1f9840a85d5af5bf1d1762f925bdaddc4201f984
        dai:0x4f96fe3b7a6cf9725f59d353f723c1bdb64ca6aa
     */
    let r30 = U256::from(11875934361112000990_u128);
    let r31 = U256::from(49991513488463610192891_u128);

    /*
    New Pools
    */
    let pair1 = Address::from_str("0xeC3c6d9adc049f88797DeC17f6aA33C4229D0C3a").unwrap();
    /*
    usdt: 0x13512979ADE267AB5100878E2e0f485B568328a4
    wbtc: 0xD1B98B6607330172f1D991521145A22BCe793277
    ratio:0.01
    */

    let n1 = U256::from(1000000_u128);
    let n2 = U256::from(100000000_u128);

    let pair2 = Address::from_str("0xE95aE0e7Ba2308aE628eAb88482657970e9EF321").unwrap();
    /*
    usdt: 0x13512979ADE267AB5100878E2e0f485B568328a4
    usdc: 0xc2569dd7d0fd715B054fBf16E75B001E5c0C1115
    ratio:1
    */
    let n3 = U256::from(1000000_u128);
    let n4 = U256::from(1000000_u128);

    let pair3 = Address::from_str("0x6c962C0B2c888Eb000D5Dd7d86af8B5c41575d45").unwrap();
    /*
    usdc: 0xc2569dd7d0fd715b054fbf16e75b001e5c0c1115
    wbtc: 0xD1B98B6607330172f1D991521145A22BCe793277
    ratio:0.01
    */ 
 
    let n5 = U256::from(1000000_u128);
    let n6 = U256::from(100000000_u128);

    let pair4 = Address::from_str("0x65cEAaDb0e0dB301b989Af173d497A4e26cB292B").unwrap();
    /*
    usdc: 0xc2569dd7d0fd715B054fBf16E75B001E5c0C1115
    weth: 0xd0A1E359811322d97991E03f863a0C30C2cF029C

    */
    let n7 = U256::from(501530872_u128);
    let n8 = U256::from(146027963733935_u128);

    let pair5 = Address::from_str("0xFe77F3cC8F3d4a1cDFeF7CED8e276aC816A42F07").unwrap();
    /*
    uni: 0x075A36BA8846C6B6F53644fDd3bf17E5151789DC
    usdc: 0xc2569dd7d0fd715B054fBf16E75B001E5c0C1115
    ratio
    */

    let n9 = U256::from(1000000000000000000_u128);
    let n10 = U256::from(1000000_u128);

    let pair6 = Address::from_str("0x39140e356ae52d77000697E4A15b8dFB911a4467").unwrap();
    /*
    uni: 0x075a36ba8846c6b6f53644fdd3bf17e5151789dc
    wbtc: 0xD1B98B6607330172f1D991521145A22BCe793277
    */

    let n11 = U256::from(2000000000000000000_u128);
    let n12 = U256::from(50000000_u128);

    let pair7 = Address::from_str("0xC4488277D4cD18C661195A07959bc1e0FE572d3E").unwrap();
    /*
    link: 0xa36085F69e2889c224210F603D836748e7dC0088
    usdt: 0x13512979ADE267AB5100878E2e0f485B568328a4
    ratio:1
    */ 

    let n13 = U256::from(9093390_u128);
    let n14 = U256::from(1100000000000000000_u128);

    let pair8 = Address::from_str("0xF2f3e28Ae6A2eDB1d8959620eD68930A9e5cbEfc").unwrap();
    /*
    link: 0xa36085F69e2889c224210F603D836748e7dC0088
    usdc 0xc2569dd7d0fd715B054fBf16E75B001E5c0C1115

    */

    let n15 = U256::from(2300000000000000000_u128);
    let n16 = U256::from(869906_u128);

    let pair9 = Address::from_str("0x7Bc2718781F22a4F19aAf2099dA817BAE53B166B").unwrap();
    /*
    dai: 0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa
    wbtc: 0xc2569dd7d0fd715B054fBf16E75B001E5c0C1115
    ratio:1
    */

    let n17 = U256::from(1000000000000000000_u128);
    let n18 = U256::from(3000000_u128);

    let pair10 = Address::from_str("0xc03c369d3BfbEE925253879b8C721Ae29b86D9a9").unwrap();
    /*
    usdt: 0x13512979ADE267AB5100878E2e0f485B568328a4
    uni: 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984
    ratio:1
    */

    let n19 = U256::from(1000000_u128);
    let n20 = U256::from(1000000000000000000_u128);

    let pair11 = Address::from_str("0x06F333Ff3BC03f56cc4411FC76dD46B92cB583Cc").unwrap();
    /*
    usdt: 0x13512979ADE267AB5100878E2e0f485B568328a4
    aave: 00xB597cd8D3217ea6477232F9217fa70837ff667Af
    ratio:1
    */ 

    let n21 = U256::from(1000000_u128);
    let n22 = U256::from(1000000000000000000_u128);

    let pair12 = Address::from_str("0x8fd9835a2578c093b260b4fBc9a7aCf4766D8032").unwrap();
    /*
    aave: 0xB597cd8D3217ea6477232F9217fa70837ff667Af
    usdc: 0xc2569dd7d0fd715B054fBf16E75B001E5c0C1115
    ratio:1
    */

    let n23 = U256::from(100000000000000000_u128);
    let n24 = U256::from(1000000_u128);

    let pair13 = Address::from_str("0x27237fC39D5B10B0092ba7501f32bcDc8BF1437B").unwrap();
    /*
    dai: 0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa
    aave: 0xB597cd8D3217ea6477232F9217fa70837ff667Af
    ratio:1
    */

    let n25 = U256::from(1000000000000000000_u128);
    let n26 = U256::from(10000000000000000_u128);

       
    /* Set Reserves */

    /* 
    use std::env;
    let args: Vec<String> = env::args().collect();
    
    let a1 = U256::from(*&args[2].parse::<u128>().unwrap());  
    let b1 = U256::from(*&args[3].parse::<u128>().unwrap()); 

    let a2 = U256::from(*&args[4].parse::<u128>().unwrap());
    let b2 = U256::from(*&args[5].parse::<u128>().unwrap());

    let a3 = U256::from(*&args[6].parse::<u128>().unwrap());
    let b3 = U256::from(*&args[7].parse::<u128>().unwrap());
*/
 

// delta_b -> delta_a,  delta_c -> delta_b, delta_c -> delta_a
// Start 41043164 (wbtc) in -> 900.. (uni) out [2 -> 1]
//  709853 USDC  in -> 41442523 (wbtc) out [3->2]
// 556667 UNI out -> 356... WBTC in    wbtc -> uni, usdc->wbtc, usdc->uni
/*
let a1 = n12; 
let b1 = n11;

let a2 =n6;
let b2 =n5;

let a3 = n10;
let b3 = n9;
        let delta_a_amt_out = SequenceToken::get_amount_out(U256::from(20025485092331716_u128), a1, b1);
        let delta_a_amt_in = SequenceToken::get_amount_in(delta_a_amt_out.unwrap(), a1, b1);

        let delta_b_amt_out = SequenceToken::get_amount_out(delta_a_amt_out.unwrap(), a2, b2);
        let delta_b_amt_in = SequenceToken::get_amount_in(delta_b_amt_out.unwrap(), a2, b2);

        let delta_c_amt_out = SequenceToken::get_amount_out(delta_b_amt_out.unwrap(), a3, b3);
        let delta_c_amt_in = SequenceToken::get_amount_in(delta_c_amt_out.unwrap(), a3, b3);
let d_a_prime_minus_a = if d_a > d_a_prime {
            U256::zero()
        } else {
            d_a_prime - d_a
        };
        let delta_c_minus_a = if delta_a > delta_c_amt_out.unwrap() {
            U256::zero()
        } else {
            delta_c_amt_out.unwrap() - delta_a
        };
        println!(
                    "\nratios - {:.5}, {:.5}, {:.5} - Profit %: {:.2}\ndelta_a: {:?} \t delta_b: {:?} \t delta_c: {:?} \t delta_a_prime: {:?} \t profit: {} \ndelta_a: {} \t delta_b: {}\t delta_c: {} \t delta_a_prime: {} \t profit: {}\n",
                    BigDecimal::from_str(&a1.to_string()).unwrap()/BigDecimal::from_str(&b1.to_string()).unwrap(),BigDecimal::from_str(&a2.to_string()).unwrap()/BigDecimal::from_str(&b2.to_string()).unwrap(),BigDecimal::from_str(&a3.to_string()).unwrap()/BigDecimal::from_str(&b3.to_string()).unwrap(),BigDecimal::from_str(&delta_c_minus_a.to_string()).unwrap()/BigDecimal::from_str(&delta_a_prime.to_string()).unwrap(),delta_a, delta_b, delta_c, delta_a_prime, profit,delta_a, delta_a_amt_out.unwrap(),delta_b_amt_out.unwrap(), delta_c_amt_out.unwrap(), delta_c_minus_a);

              
        let trade1 = SwapRoute::new(
            (id_a1, id_b1),
            U256::from(900132399311970944_u128),
            U256::from(900132399311970944_u128),
            U256::zero(),
            U256::zero(),
            *kovan::router_v2,
            uni2_wbtc_pair,
            usdc2_wbtc_pair
        );

        let trade2 = SwapRoute::new(
            (id_a2, id_b2),
            delta_c,
            U256::from(41442523_u128),
            U256::zero(),
            U256::zero(),
             *kovan::router_v2,
            usdc2_wbtc_pair,
            uni2_usdc2_pair,
        );

        let trade3 = SwapRoute::new(
            (id_a3, id_b3),
            delta_a_prime,
            U256::from(35691209156324608_u128),
            U256::zero(),
            U256::zero(),
            *kovan::router_v2,
            uni2_usdc2_pair,
            uni2_weth_pair

        );

        let trade_vec = vec![trade1, trade2, trade3];

        let flash_token = IERC20::new(id_a1, kovan::client.clone());
        let flash_repayment =
            flash_token.transfer(first_pair.clone(), delta_a.mul(U256::from(997_i16)));

        let calls: Vec<
            ethers::prelude::builders::ContractCall<
                ethers::prelude::SignerMiddleware<
                    ethers::prelude::Provider<ethers::prelude::Http>,
                    ethers::prelude::Wallet<ethers::prelude::k256::ecdsa::SigningKey>,
                >,
                bool,
            >,
        > = vec![flash_repayment];
//
//99999999999999922038850000000000
//100000000000000000000000000000000

        let tx_out = SwapRoute::route_calldata(trade_vec, calls).await.unwrap();
        println!("{:#02x}", tx_out);
        /*
            let flash_tx: TypedTransaction = flash_swap_v2(
                first_pair.clone(),
                delta_a,
                delta_b,
                SwapRoute::route_calldata(trade_vec, calls).await.unwrap(),
            )
            .await
            .unwrap();
        */

        //println!("Flash Tx: {}", &flash_tx.data().unwrap());
         */
    }


#[tokio::test]
pub async fn test_cyclic_order() -> Result<(), anyhow::Error> {
    let pair1 = CryptoPair::new(DexPool {
        id: Address::from_str("0xd6f3768e62ef92a9798e5a8cedd2b78907cecef9").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x6243d8cea23066d098a15582d81a598b4e8391f4").unwrap(),
            symbol: "FLX".to_string(),
            name: "Flex".to_string(),
            decimals: 18,
            reserve: U256::from_dec_str("84732766374753429281750").unwrap(),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from_dec_str("2270162188516010141072").unwrap(),
        },
    });

    let pair2 = CryptoPair::new(DexPool {
        id: Address::from_str("0xd6ef070951d008f1e6426ad9ca1c4fcf7220ee4d").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0x3ea8ea4237344c9931214796d9417af1a1180770").unwrap(),
            symbol: "FLX".to_string(),
            name: "FLX".to_string(),
            decimals: 18,
            reserve: U256::from_dec_str("9850589243410936446090896").unwrap(),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
            symbol: "USDC".to_string(),
            name: "USDC".to_string(),
            decimals: 6,
            reserve: U256::from_dec_str("1208409881187").unwrap(),
        },
    });

    let pair3 = CryptoPair::new(DexPool {
        id: Address::from_str("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc").unwrap(),
        sqrt_price: U256::zero(),
        liquidity: U256::zero(),
        fee_tier: 0,
        tick: 0,
        dex: "uni_v2".to_string(),
        router: Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d").unwrap(),
        token0: UniswapPairsPairsTokens {
            id: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
            symbol: "USDC".to_string(),
            name: "USDC".to_string(),
            decimals: 6,
            reserve: U256::from_dec_str("51272280872868").unwrap(),
        },
        token1: UniswapPairsPairsTokens {
            id: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(),
            symbol: "WETH".to_string(),
            name: "WETH".to_string(),
            decimals: 18,
            reserve: U256::from_dec_str("50084906071988998561924").unwrap(),
        },
    });

    let mut crypto_pairs: HashMap<Address, Arc<CryptoPair>> = HashMap::new();
    let crypto_paths = vec![pair1.clone(), pair2.clone(), pair3.clone()];

    crypto_pairs.insert(*pair1.pair_id(), Arc::new(pair1.clone()));
    crypto_pairs.insert(*pair2.pair_id(), Arc::new(pair2.clone()));
    crypto_pairs.insert(*pair3.pair_id(), Arc::new(pair3.clone()));

    let sequence = three_path_sequence::cyclic_order(crypto_paths, &crypto_pairs)
        .await
        .unwrap();
    let sequence = sequence
        .downcast_ref::<ThreePathSequence>()
        .unwrap()
        .clone();

    println!(
        "a1: {}, b1: {}, a2: {}, b2: {}, a3: {}, b3: {}",
        sequence.a1().symbol(),
        sequence.b1().symbol(),
        sequence.a2().symbol(),
        sequence.b2().symbol(),
        sequence.a3().symbol(),
        sequence.b3().symbol()
    );
    assert!(sequence.a1().symbol() == sequence.b3().symbol());
    assert!(sequence.b1().symbol() == sequence.a2().symbol());
    assert!(sequence.b2().symbol() == sequence.a3().symbol());

    let result = optimize_a_prime_2(
        &sequence.a1().pending_reserve(),
        &sequence.b1().pending_reserve(),
        &sequence.a2().pending_reserve(),
        &sequence.b2().pending_reserve(),
        &sequence.a3().pending_reserve(),
        &sequence.b3().pending_reserve(),
    );

    let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();
    let delta_a_rat = delta_a.to_big_rational();
    let delta_b_rat = delta_b.to_big_rational();
    let delta_c_rat = delta_c.to_big_rational();
    let delta_a_prime_rat = delta_a_prime.to_big_rational();

    let delta_a_dec = ratio_to_dec(delta_a_rat).div(
        BigDecimal::from_u128(10_u128.pow(sequence.a1().decimal().try_into().unwrap())).unwrap(),
    );
    let delta_b_dec = ratio_to_dec(delta_b_rat).div(
        BigDecimal::from_u128(10_u128.pow(sequence.a2().decimal().try_into().unwrap())).unwrap(),
    );
    println!("delta_c: {}", delta_c);
    let delta_c_dec = ratio_to_dec(delta_c_rat).div(
        BigDecimal::from_u128(10_u128.pow(sequence.a3().decimal().try_into().unwrap())).unwrap(),
    );
    let delta_a_prime_dec = ratio_to_dec(delta_a_prime_rat).div(
        BigDecimal::from_u128(10_u128.pow(sequence.a1().decimal().try_into().unwrap())).unwrap(),
    );

    println!(
        "\r
        Pair 1 - {} 
        \t\t{} - Reserve:{}\t\tDecimal:{}\tPrice: {:.6}\tFormatted: {}
        \t\t{} - Reserve:{}\t\tDecimal:{}\tPrice: {:.6}\tFormatted: {}
        Pair 2 - {} 
        \t\t{} - Reserve:{}\t\tDecimal:{}\tPrice: {:.6}\tFormatted: {}
        \t\t{} - Reserve:{}\t\tDecimal:{}\tPrice: {:.6}\tFormatted: {}
        Pair 3 - {} 
        \t\t{} - Reserve:{}\t\tDecimal:{}\tPrice: {:.6}\tFormatted: {}
        \t\t{} - Reserve:{}\t\tDecimal:{}\tPrice: {:.6}\tFormatted: {}
        \r\r
        Trade Setup - optimize_a_prime
        \r
        Trade {:.6} {} for {:.6} {} at price {:.5} - to_amount: {:.5} to_reserve: {}
        Trade {:.6} {} for {:.6} {} at price {:.5} - to_amount: {:.5} to_reserve: {}
        Trade {:.6} {} for {:.6} {} at price {:.5} - to_amount: {:.5} to_reserve: {}
        \r", /*
             Trade Setup - get_amounts_in/out
             \r
             Trade {} {} for {} {} at price {}
             Trade {} {} for {} {} at price {}
             Trade {} {} for {} {} at price {}
             \r", */
        &sequence.a1().token.pair_symbol(),
        /*
        /*
            Method: optimize_a_prime  Profit: -4829.110
                        Trade 4834.97 WETH for 55534.16 FLX at price 0.028
                                    WETH Reserves:  2284 Ratio: 0.03  FLX Reserves:  81847 Ratio: 35.835
                        Trade 55534.16 FLX for 7043.65 USDC at price 0.000
                                    FLX Reserves:  9704226 Ratio: 7.82  USDC Reserves:  1241578 Ratio: 0.128
                        Trade 7043.65 USDC for 5.86 WETH at price 1199200326215895.250
        */

            1 FLX = 0.0267 ETH ($28.24)
            1 ETH = 37.4017 FLX ($1,056)  // Amounts are correct.  Not Price

            1 FLX = 0.1227 USDC ($0.12) // 12 decimals off to the right
            1 USDC = 8.1517 FLX ($1.00) // 12 decimals off to left
            // Both have an 18 / 6 split on decimals
            1 USDC = 0.0009 ETH ($1.00) // 12 off to the left
            1 ETH = 1,056 USDC ($1,056) // 12 off to the right

            Pair 1 - FLXWETH
                            WETH - Reserve:2270162188516010141072 Decimal:18 Price: 37.32454306718176496568693526604978291268162011473524822305513254301916147219478944996577464378938169 Rational: 42366383187376714640875/1135081094258005070536 Reduced: 42366383187376714640875/1135081094258005070536
                            FLX - Reserve:84732766374753429281750 Decimal:18 Price: 0.02679202256274282145221256743342700240977157232632748274774988887297292816224031394265651044888296089 Rational: 0 Reduced: 0
            Pair 2 - FLXUSDC
                            FLX - Reserve:9850589243410936446090896 Decimal:18 Price: 0.0000000000001226738676567298662254166755044285748803960549064541509257602478379823980853439008869624353521904324 Rational: 0 Reduced: 0
                            USDC - Reserve:1208409881187 Decimal:6 Price: 8151695378173.235419259618729150768420147340971165434585181171431162040409013089196607249788149029782 Rational: 8151695378173 Reduced: 8151695378173
            Pair 3 - USDCWETH
                            USDC - Reserve:51272280872868 Decimal:6 Price: 976841779.2096444399734140352113469093463767543133797680927945584705536663656221714105966515469045254 Rational: 976841779 Reduced: 976841779
                            WETH - Reserve:50084906071988998561924 Decimal:18 Price: 0.000000001023707238248033072038333415816150754748536187721595028407680792969441920632272118189191009739614706 Rational: 0 Reduced: 0

            Trade Setup - optimize_a_prime

            Trade 16017472942892544 WETH for 5918876993148546029.24 FLX at price 0.37
            Trade 5918876993148546029.24 FLX for 682997431.37 USDC at price 0.00
            Trade 682997431.37 USDC for 285717182798116995619.73 WETH at price 9768417.79
        */
        &sequence.a1().symbol(),
        &sequence.a1().pending_reserve(),
        &sequence.a1().decimal(),
        &sequence.a1().decimal_price().unwrap().to_string(),
        format_units(
            &sequence.a1().pending_reserve(),
            sequence.a1().decimal() as usize
        ),
        &sequence.b1().symbol(),
        &sequence.b1().pending_reserve(),
        &sequence.b1().decimal(),
        &sequence.b1().decimal_price().unwrap().to_string(),
        format_units(
            &sequence.b1().pending_reserve(),
            sequence.b1().decimal() as usize
        ),
        &sequence.a2().token.pair_symbol(),
        /*
            1 FLX = 0.1227 USDC ($0.12)
            1 USDC = 8.1517 FLX ($1.00)
        */
        &sequence.a2().symbol(),
        &sequence.a2().pending_reserve(),
        &sequence.a2().decimal(),
        &sequence.a2().decimal_price().unwrap().to_string(),
        format_units(
            &sequence.a2().pending_reserve(),
            sequence.a2().decimal() as usize
        ),
        &sequence.b2().symbol(),
        &sequence.b2().pending_reserve(),
        &sequence.b2().decimal(),
        &sequence.b2().decimal_price().unwrap().to_string(),
        format_units(
            &sequence.b2().pending_reserve(),
            sequence.b2().decimal() as usize
        ),
        &sequence.a3().token.pair_symbol(),
        /*
            1 USDC = 0.0009 ETH ($1.00)
            1 ETH = 1,056 USDC ($1,056)
        */
        &sequence.a3().symbol(),
        &sequence.a3().pending_reserve(),
        &sequence.a3().decimal(),
        &sequence.a3().decimal_price().unwrap().to_string(),
        &sequence
            .a3()
            .rational_price()
            .unwrap()
            .reduced()
            .to_integer(),
        &sequence.b3().symbol(),
        &sequence.b3().pending_reserve(),
        &sequence.b3().decimal(),
        &sequence.b3().decimal_price().unwrap().to_string(),
        &sequence
            .b3()
            .rational_price()
            .unwrap()
            .reduced()
            .to_integer(),
        &delta_a_dec,
        &sequence.a1().symbol(),
        &delta_b_dec,
        &sequence.b1().symbol(),
        format_units(
            &big_rational_to_u256(&sequence.a1().rational_price().unwrap()).unwrap(),
            5
        ),
        &sequence.a1().to_amount(&delta_a).unwrap(),
        &sequence
            .a1()
            .to_reserve(&sequence.a1().to_amount(&delta_a).unwrap())
            .unwrap(),
        delta_b_dec,
        &sequence.a2().symbol(),
        &delta_c_dec,
        &sequence.b2().symbol(),
        format_units(
            &big_rational_to_u256(&sequence.a2().rational_price().unwrap()).unwrap(),
            5
        ),
        &sequence.a2().to_amount(&delta_b).unwrap(),
        &sequence
            .a2()
            .to_reserve(&sequence.a2().to_amount(&delta_b).unwrap())
            .unwrap(),
        &delta_c_dec,
        &sequence.a3().symbol(),
        &delta_a_prime_dec,
        &sequence.b3().symbol(),
        format_units(
            &big_rational_to_u256(&sequence.a3().rational_price().unwrap()).unwrap(),
            5
        ),
        &sequence.a3().to_amount(&delta_c).unwrap(),
        &sequence
            .a3()
            .to_reserve(&sequence.a2().to_amount(&delta_c).unwrap())
            .unwrap(),
        /*
        format_units(&delta_a, 2),
        &sequence.a1().symbol(),
        format_units(&sequence.b1().amount_out(delta_a.clone()).unwrap().0, 2),
        &sequence.b1().symbol(),
        format_units(&big_rational_to_u256(&sequence.a1().rational_price().unwrap()).unwrap(),2),

        format_units(&sequence.a2().amount_in(sequence.b1().amount_out(delta_a.clone()).unwrap().0).unwrap().0, 2),
        &sequence.a2().symbol(),
        format_units(&sequence.b2().amount_out(sequence.a2().amount_in(sequence.b1().amount_out(delta_a.clone()).unwrap().0).unwrap().0).unwrap().0, 2),
        &sequence.b2().symbol(),
        format_units(&big_rational_to_u256(&sequence.a2().rational_price().unwrap()).unwrap(),2),

        format_units(&sequence.a3().amount_in(sequence.b2().amount_out(sequence.a2().amount_in(sequence.b1().amount_out(delta_a.clone()).unwrap().0).unwrap().0).unwrap().0).unwrap().0, 2),
        &sequence.a3().symbol(),
        format_units(&sequence.b3().amount_out(sequence.a3().amount_in(sequence.b2().amount_out(sequence.a2().amount_in(sequence.b1().amount_out(delta_a.clone()).unwrap().0).unwrap().0).unwrap().0).unwrap().0).unwrap().0, 2),
        &sequence.b3().symbol(),
        format_units(&big_rational_to_u256(&sequence.a3().rational_price().unwrap()).unwrap(),2)*/
    );

    Ok(())
    /*
        &sequence.b1().symbol(),
        (BigDecimal::from_str(&*sequence.a1().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.b1().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap(),
        sequence.a1().symbol(),
        sequence.a1().pending_reserve(),
        (BigDecimal::from_str(&*sequence.a1().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.b1().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap(),
        sequence.b1().symbol(),
        sequence.b1().pending_reserve(),
        (BigDecimal::from_str(&*sequence.b1().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.a1().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap(),
        delta_b.to_f64().unwrap(),
        sequence.a2().symbol(),
        delta_c.to_f64().unwrap(),
        sequence.b2().symbol(),
        ((BigDecimal::from_str(&*sequence.a2().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(
                &*10_i128.pow(sequence.a2().decimal() as u32).to_string()
            )
            .unwrap())
            / (BigDecimal::from_str(&*sequence.b2().pending_reserve().to_string()).unwrap()
                / BigDecimal::from_str(
                    &*10_i128.pow(sequence.b2().decimal() as u32).to_string()
                )
                .unwrap()))
        .to_f64()
        .unwrap(),
        sequence.a2().symbol(),
        sequence.a2().pending_reserve(),
        (BigDecimal::from_str(&*sequence.a2().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.b2().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap(),
        sequence.b2().symbol(),
        sequence.b2().pending_reserve(),
        (BigDecimal::from_str(&*sequence.b2().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.a2().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap(),
        delta_c.to_f64().unwrap(),
        sequence.a3().symbol(),
        delta_a_prime.to_f64().unwrap(),
        sequence.b3().symbol(),
        (BigDecimal::from_str(&*sequence.a3().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(
                &*10_i128.pow(sequence.a3().decimal() as u32).to_string()
            )
            .unwrap()
            / (BigDecimal::from_str(&*sequence.b3().pending_reserve().to_string()).unwrap()
                / BigDecimal::from_str(
                    &*10_i128.pow(sequence.b3().decimal() as u32).to_string()
                )
                .unwrap()))
        .to_f64()
        .unwrap(),
        sequence.a3().symbol(),
        sequence.a3().pending_reserve(),
        (BigDecimal::from_str(&*sequence.a3().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.b3().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap(),
        sequence.b3().symbol(),
        sequence.b3().pending_reserve(),
        (BigDecimal::from_str(&*sequence.b3().pending_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*sequence.a3().pending_reserve().to_string()).unwrap())
        .to_f64()
        .unwrap()
    );
     */
}
