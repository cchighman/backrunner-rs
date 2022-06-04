use crate::arbitrage_path::ArbitragePath;
use ethers::prelude::{Address, U256};
use futures_signals::map_ref;
use futures_signals::signal::{MutableSignal, SignalExt};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::future::ready;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

use crate::arb_thread_pool::spawn;
use crate::crypto_pair::CryptoPair;
use crate::dex_pool::DexPool;
use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;
use crate::utils::common::DIRECTION;

#[derive(Debug, Clone)]
pub struct SequenceToken {
    pub token: Arc<CryptoPair>,
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

    pub fn get_signal(&self) -> MutableSignal<U256> {
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

    pub fn router(&self) -> Address {
        self.token.pair.router
    }
}
