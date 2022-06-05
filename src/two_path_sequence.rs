pub mod two_path_sequence {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use ethers::prelude::*;
    use std::str::FromStr;
    use crate::crypto_pair::CryptoPair;
    use crate::sequence_token::SequenceToken;
    use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;
    use crate::utils::common::DIRECTION;
    use crate::dex_pool::DexPool;

#[derive(Debug, Clone)]
pub struct TwoPathSequence {
    pub(crate) sequence: Vec<SequenceToken>,
}
impl TwoPathSequence {
    pub fn new(path: Vec<SequenceToken>) -> Self {
        Self { sequence: path }
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

}

pub async fn two_cyclic_order(
    crypto_path: Vec<CryptoPair>,
    crypto_pairs: &HashMap<Address, Arc<CryptoPair>>,
) -> Option<TwoPathSequence> {
    
    /* Scenario 1 */
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    
    /* Scenario 2 */
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();

    let scenario_1 = a1_b2 && b1_a2;  // WETH-DAI-DAI-WETH (a1-b2 alike), (b1-a2 alike)
    let scenario_2 = b1_a2 && a1_b2;  // DAI-WETH-WETH-DAI  (a2-b1 alike), (a1-b2 alike)    
    
    let pair_id_1 = crypto_path[0].pair_id();
    let pair_1 = crypto_pairs.get_key_value(pair_id_1).unwrap().1;

    let pair_id_2 = crypto_path[1].pair_id();
    let pair_2 = crypto_pairs.get_key_value(pair_id_2).unwrap().1;

    let token_a1 = if scenario_1 {
        Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Left))
    } else {
        Option::from(SequenceToken::new(pair_1.clone(), DIRECTION::Right))
    };

    let token_b1 = if scenario_1  {
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

    let path = TwoPathSequence::new(vec![
        token_a1.unwrap(),
        token_b1.unwrap(),
        token_a2.unwrap(),
        token_b2.unwrap()]);
    Some(path)
}
  
pub async fn is_arbitrage_pair(crypto_path: &Vec<CryptoPair>) -> Result<bool, anyhow::Error> {
    /* Scenario 1 */
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    
    /* Scenario 2 */
    let b1_a2 = crypto_path[0].right_symbol() == crypto_path[1].left_symbol();
    let a1_b2 = crypto_path[0].left_symbol() == crypto_path[1].right_symbol();

    let scenario_1 = a1_b2 && b1_a2;  // WETH-DAI-DAI-WETH (a1-b2 alike), (b1-a2 alike)
    let scenario_2 = b1_a2 && a1_b2;  // DAI-WETH-WETH-DAI  (a2-b1 alike), (a1-b2 alike)    
    
    if scenario_1.clone() 
    || scenario_2.clone() {
    Ok(true)
} else {
    Ok(false)
}
}

pub async fn test_is_arbitrage_pair_true() {
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
}


pub async fn test_cyclic_order() {
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
    
        let ordered = two_path_sequence::two_cyclic_order(crypto_paths, &crypto_pairs).await.unwrap();

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
            
    
}



}


