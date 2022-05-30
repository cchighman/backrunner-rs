use crate::uniswapv2_pairs;
use ethereum_types::{Address, U256};
use serde::{Deserialize, Serialize};

use uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexPool {
    pub id: Address,
    pub sqrt_price: U256,
    pub liquidity: U256,
    pub fee_tier: i32,
    pub tick: i32,
    pub dex: String,
    pub router: Address,
    pub token0: UniswapPairsPairsTokens,
    pub token1: UniswapPairsPairsTokens,
}
