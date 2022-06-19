use std::sync::Arc;

use bigdecimal::BigDecimal;
use ethers::prelude::{Address, U256};
use futures_signals::signal::MutableSignal;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_rational::Ratio;

use crate::crypto_pair::CryptoPair;
use crate::uniswapv2_pairs::uniswap_pairs::UniswapPairsPairsTokens;
use crate::utils::common::DIRECTION;

use crate::utils::conversions::*;
use crate::utils::u256_decimal::*;
use crate::utils::ratio_as_decimal::*;


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

    pub fn id(&self) -> &Address {
        &self.id
    }

    pub fn pending_reserve(&self) -> U256 {
        self.token.pending_reserve(self.token_direction.clone())
    }

    pub fn confirmed_reserve(&self) -> U256 {
        self.token.confirmed_reserve(self.token_direction.clone())
    }

    pub fn pending_signal(&self) -> MutableSignal<U256> {
        self.token.pending_signal(self.token_direction.clone())
    }

    pub fn confirmed_signal(&self) -> MutableSignal<U256> {
        self.token.confirmed_signal(self.token_direction.clone())
    }

    pub fn decimal(&self) -> i32 {
        self.token_context.decimals
    }

    pub fn symbol(&self) -> &String {
        &self.token_context.symbol
    }

    pub fn direction(&self) -> &DIRECTION {
        return &self.token_direction;
    }

    pub fn rational_price(&self)->Result<Ratio<BigInt>, anyhow::Error> {
        return self.token.rational_price(self.token_direction.clone());
    }

    pub fn decimal_price(&self)->Result<BigDecimal, anyhow::Error> {
        return self.token.decimal_price(self.token_direction.clone());
    }

    pub fn to_amount(&self, reserve_amt: &U256)->Result<BigDecimal, anyhow::Error> {
        return self.token.to_amount(reserve_amt, self.token_direction.clone());
    }

    pub fn to_reserve(&self, token_amt: &BigDecimal)->Result<U256, anyhow::Error> {
        return self.token.to_reserve(token_amt, self.token_direction.clone());
    }

    pub fn router(&self) -> Address {
        self.token.pair.router
    }
}
