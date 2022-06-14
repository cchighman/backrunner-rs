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
use crate::arb_thread_pool::spawn;
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
pub struct TwoPathSequence {
    pub(crate) sequence: Vec<SequenceToken>,
}
/* 
pub async fn cyclic_order(
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: &HashMap<Address, Arc<CryptoPair>>
)->Result<Arc<(dyn Any + 'static + Sync + Send)>, anyhow::Error>  {
    
    /* Scenario 1 */
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    
    /* Scenario 2 */
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();

    /* Scenario 3 */
    let a1_b1 = crypto_path[0].left_symbol() == crypto_path[1].left_symbol();
    let a2_b2 = crypto_path[0].right_symbol() == crypto_path[1].right_symbol();
  
    let scenario_1 = a1_b2 && b1_a2;  // WETH-DAI-DAI-WETH (a1-b2 alike), (b1-a2 alike)
    let scenario_2 = b1_a2 && a1_b2;  // DAI-WETH-WETH-DAI  (a2-b1 alike), (a1-b2 alike)
    let scenario_3 = a1_b1 && a2_b2;  // WETH-DAI-WETH_DAI
    
    let pair_id_1 = crypto_path[0].pair_id();
    let pair_1 = crypto_pairs.get_key_value(pair_id_1).unwrap().1;

    let pair_id_2 = crypto_path[1].pair_id();
    let pair_2 = crypto_pairs.get_key_value(pair_id_2).unwrap().1;

    let token_a1 = if scenario_1 || scenario_3 {
        Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Left))
    } else {
        Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Right))
    };

    let token_b1 = if scenario_1 || scenario_3 {
        Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Right))
    } else {
        Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Left))
    };

    let token_a2 = if scenario_1 {
        Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Right))
    } else {
        Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Left))
    };

    let token_b2 = if scenario_1 {
        Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Left))
    } else {
        Option::from(SequenceToken::new(pair_2.clone(), DIRECTION::Right))
    };
/* 
    let path = TwoPathSequence::new(vec![
        token_a1.unwrap(),
        token_b1.unwrap(),
        token_a2.unwrap(),
        token_b2.unwrap()]);
  Ok(path.await)*/
  
}
*/



pub fn is_arbitrage_pair(crypto_path: &Vec<CryptoPair>)->bool {
    /* Scenario 1 */
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    
    /* Scenario 2 */
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();

    /* Scenario 3 */
    let a1_b1 = crypto_path[0].left_symbol() == crypto_path[1].left_symbol();
    let a2_b2 = crypto_path[0].right_symbol() == crypto_path[1].right_symbol();

    let scenario_1 = a1_b2 && b1_a2;  // WETH-DAI-DAI-WETH (a1-b2 alike), (b1-a2 alike)
    let scenario_2 = b1_a2 && a1_b2;  // DAI-WETH-WETH-DAI  (a2-b1 alike), (a1-b2 alike)    
    let scenario_3 = a1_b1 && a2_b2; // WETH-DAI-WETH_DAI
    if scenario_1.clone() 
    || scenario_2.clone() || scenario_3.clone() {
    true
} else {
    false
}
}

/*
pub(crate) async fn test_is_arbitrage_pair_true() {
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
        }
    });
   
    let arb_vec = vec![pair1, pair2];
    println!("Result: {}", two_path_sequence::is_arbitrage_pair(&arb_vec).await.unwrap());
    assert!(two_path_sequence::is_arbitrage_pair(&arb_vec).await.unwrap());
    //let path = TwoPathSequence::new(arb_vec);
    //let arb_path = ArbitragePath::from_path_sequence(path);

}
*/

impl TwoPathSequence {
 
    fn path(&self) -> String {
        let mut path_str: String = Default::default();
        for token in 0..self.sequence.len() {
            path_str = path_str.to_owned() + " - " + self.sequence[token].get_symbol();
        }
        path_str
    }
    /* 
    pub async fn calculate(sequence: Arc<TwoPathSequence>) {
       
        let result = optimize_a_prime(
            BigDecimal::from_str(&*sequence.a1().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b1().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.a2().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b2().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.a3().get_reserve().to_string()).unwrap(),
            BigDecimal::from_str(&*sequence.b3().get_reserve().to_string()).unwrap(),
        );

        if !result.is_none() {
            let (delta_a, delta_b, delta_c, delta_a_prime, profit) = result.unwrap();
            let method = "optimize_a_prime";
            println!(
                "Method: {}  Profit: {:.3?}
                    Trade {:.2?} {} for {:.2?} {} at price {:.3?}
                    \t\t{} Reserves:  {} Ratio: {:.2?}  {} Reserves:  {:.3?} Ratio: {:.3?}
                    Trade {:.2?} {} for {:.2?} {} at price {:.3?}
                    \t\t{} Reserves:  {:.3?} Ratio: {:.2?}  {} Reserves:  {:.3?} Ratio: {:.3?}"
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
            let bundle_result = send_flashswap_bundle(flash_tx).await;
            if bundle_result.as_ref().is_err() {
                println!("Flash bundle could not be submitted.  Reason: {:#}", bundle_result.as_ref().err().unwrap());
            } else {
                let bundle_hash = bundle_result.as_ref().unwrap();
                println!("### Successful Bundle - tx hash: {:#}", &bundle_hash);
            }
        
        }
        */ 
    }



/*
#[async_trait]
impl PathSequence for TwoPathSequence {
    async fn new(path: Vec<SequenceToken>)->Arc<(dyn Any + 'static + Sync + Send)> {
        Arc::new(Self{sequence: path})
    }      
} 
 */
/* 
    fn arb_index(&self)-> BigDecimal {
        (BigDecimal::from_str(&*self.a1().get_reserve().to_string()).unwrap()
            / BigDecimal::from_str(&*self.b1().get_reserve().to_string()).unwrap())
            * (BigDecimal::from_str(&*self.a2().get_reserve().to_string()).unwrap()
                / BigDecimal::from_str(&*self.b2().get_reserve().to_string()).unwrap())
    }

    async fn init(&self, arb_ref: Arc<(dyn Any + 'static + Sync + Send)>)->Result<(),anyhow::Error> {
        let seq = arb_ref.downcast_ref::<TwoPathSequence>().unwrap().clone();
        let t = map_ref! {
            let a1 = self.a1().get_signal(),
             let b1 =self.b1().get_signal(),
             let a2 =  self.a2().get_signal(),
             let b2 =  self.b2().get_signal() =>
             (BigDecimal::from_str(&a1.to_string()).unwrap()
            / BigDecimal::from_str(&*b1.to_string()).unwrap())
            * (BigDecimal::from_str(&*a2.to_string()).unwrap()
                / BigDecimal::from_str(&*b2.to_string()).unwrap())
        };
    
        let future = t.for_each(move |v| {
            println!(
                "Arb Index -- path: {} Index: {:.3?}",
                seq.path(),
                v.to_f64().unwrap()
            );
    
            if v > BigDecimal::from_f64(1.05).unwrap() {
                spawn(TwoPathSequence::calculate(Arc::new(seq.clone())));
            }
            ready(())
        });
        Ok(spawn(future))
    }
    
    fn as_any(&self)->&dyn Any {
        self
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
}


#[tokio::test]
async fn test_cyclic_order() {
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
                symbol: "DAI".to_string(),
                name: "DAI".to_string(),
                decimals: 18,
                reserve: U256::from(1000000),
            }
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
            }
        });

        let mut crypto_pairs: HashMap<Address, Arc<CryptoPair>> = HashMap::new();
        let mut crypto_paths = vec![pair1.clone(), pair2.clone()];
    
        crypto_pairs.insert(pair1.pair_id().clone(), Arc::new(pair1.clone()));
        crypto_pairs.insert(pair2.pair_id().clone(), Arc::new(pair2.clone()));
    
        let ordered = self.cyclic_order(crypto_paths, &crypto_pairs).await.unwrap();

        println!(
            "a1: {}, b1: {}, a2: {}, b2: {}",
            ordered.a1().get_symbol(),
            ordered.b1().get_symbol(),
            ordered.a2().get_symbol(),
            ordered.b2().get_symbol()
        );
        /* Scenario 1 Ordering */
        assert!(ordered.a1().get_symbol() == ordered.b2().get_symbol());
        assert!(ordered.b1().get_symbol() == ordered.a2().get_symbol());

        /* Scenario 1 Ordering */
        assert!(ordered.b1().get_symbol() == ordered.a2().get_symbol());
        assert!(ordered.a1().get_symbol() == ordered.b2().get_symbol());
        Ok(())
    
}
  */