use crate::arb_thread_pool::spawn;
use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use bigdecimal::BigDecimal;
use ethereum_types::{Address, U256};
use futures::executor;
use futures_signals::map_ref;
use futures_signals::signal::{MutableSignal, SignalExt};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::ready;
use std::mem::swap;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;

pub fn dec_to_int(dec_string: &str, places: i64) -> U256 {
    let rounded = BigDecimal::from_str(dec_string).unwrap().round(places);
    //  println!("{}", rounded);
    let base: i128 = 10;
    let num_exp = BigInt::from(base.pow(places as u32));
    //println!("{}", num_exp);
    let result = rounded.mul(num_exp);
    // println!("{}", result.normalized());

    return U256::from_dec_str(result.normalized().to_string().as_str()).unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub enum DIRECTION {
    Default,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct SequenceToken {
    token: Arc<CryptoPair>,
    token_direction: DIRECTION,
    token_context: UniswapPairsPairsTokens,
    id: Address,
}

impl SequenceToken {
    pub fn new(new_token: Arc<CryptoPair>, direction: DIRECTION) -> Self {
        let context: UniswapPairsPairsTokens = if direction == DIRECTION::Left {
            new_token.pair.token0.clone()
        } else {
            new_token.pair.token1.clone()
        };

        Self {
            token: new_token.clone(),
            token_direction: direction,
            token_context: context.clone(),
            id: context.id.clone(),
        }
    }

    pub fn get_id(&self) -> &Address {
        &self.id
    }

    pub fn get_reserve(&self) -> U256 {
        self.token.get_reserve(self.token_direction.clone())
    }

    pub fn get_signal(&self) -> &MutableSignal<U256> {
        self.token.get_signal(self.token_direction.clone())
    }

    pub fn get_decimal(&self) -> i32 {
        self.token_context.decimals
    }

    pub fn get_symbol(&self) -> &String {
        &self.token_context.symbol
    }

    pub fn get_direction(&self) -> &DIRECTION {
        return &self.token_direction;
    }
}

#[derive(Debug, Clone)]
pub struct ThreePathSequence {
    pub(crate) sequence: Vec<SequenceToken>,
}
impl ThreePathSequence {
    pub fn new(path: Vec<SequenceToken>) -> Self {
        Self { sequence: path }
    }

    pub async fn init(&self) {
        let mut t = map_ref! {
            let a1 =self.a1().token.left_reserves_signal(),
            let b1 = self.a1().token.right_reserves_signal(),
            let a2 =self.b1().token.left_reserves_signal(),
            let b2 = self.b1().token.right_reserves_signal()
            =>
            a1 + b1 + a2 + b2
        };

        let future = t.for_each(move |v| {
            println!("Arb Index -- {}", v);

            ready(())
        });
        spawn(future);
    }

    pub fn a1(&self) -> &SequenceToken {
        &self.sequence[0]
    }
    pub fn b1(&self) -> &SequenceToken {
        &self.sequence[1]
    }

    pub fn a2(&self) -> &SequenceToken {
        &self.sequence[2]
    }
    pub fn b2(&self) -> &SequenceToken {
        &self.sequence[3]
    }
    pub fn a3(&self) -> &SequenceToken {
        &self.sequence[4]
    }
    pub fn b3(&self) -> &SequenceToken {
        &self.sequence[5]
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

pub fn cyclic_order(
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: HashMap<Address, Arc<CryptoPair>>,
) -> Option<ThreePathSequence> {
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

    let mut path = ThreePathSequence::new(vec![
        token_a1.unwrap(),
        token_b1.unwrap(),
        token_a2.unwrap(),
        token_b2.unwrap(),
        token_a3.unwrap(),
        token_b3.unwrap(),
    ]);
    return Some(path);
}

#[test]
pub fn test_is_arbitrage_pair_true() {
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

    let arb_vec = vec![pair1, pair2, pair3];
    println!("Result: {}", is_arbitrage_pair(&arb_vec));
    assert!(is_arbitrage_pair(&arb_vec));
}

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
    println!("Result: {}", is_arbitrage_pair(&arb_vec));
    assert!(!is_arbitrage_pair(&arb_vec));
}

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

    let ordered = cyclic_order(crypto_paths, crypto_pairs.clone()).unwrap();

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
}
