use std::ops::Add;
use std::ops::Div;
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
use crate::utils::ratio_as_decimal::*;
use crate::utils::u256_decimal::*;

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

    pub fn pair_id(&self) -> &Address {
        &self.token.pair.id
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

    pub fn decimal_price_18_digits(&self) -> Result<BigDecimal, anyhow::Error> {
        return self
            .token
            .decimal_price_18_digits(self.token_direction.clone());
    }

    pub fn rational_price(&self) -> Result<Ratio<BigInt>, anyhow::Error> {
        return self.token.rational_price(self.token_direction.clone());
    }

    pub fn decimal_price(&self) -> Result<BigDecimal, anyhow::Error> {
        return self.token.decimal_price_2(self.token_direction.clone());
    }

    pub fn to_amount(&self, reserve_amt: &U256) -> Result<BigDecimal, anyhow::Error> {
        return self
            .token
            .to_amount(reserve_amt, self.token_direction.clone());
    }

    pub fn to_reserve(&self, token_amt: &BigDecimal) -> Result<U256, anyhow::Error> {
        return self
            .token
            .to_reserve(token_amt, self.token_direction.clone());
    }

    pub fn amount_in(&self, amount_out: &BigDecimal) -> Option<(BigDecimal, Address)> {
        let a = self.token.amount_in(amount_out, self.direction().clone())?;
        let top_bytes = a.0.numer().to_bytes_le();
        let top = BigInt::from_bytes_le(top_bytes.0, &top_bytes.1);
    
        let bottom_bytes = a.0.denom().to_bytes_le();
        let bottom = BigInt::from_bytes_le(bottom_bytes.0, &bottom_bytes.1);
        let decimal = BigDecimal::from(top) / BigDecimal::from(bottom);
        Some((decimal,a.1))
    }

    pub fn amount_out(&self, amount_in: &BigDecimal) -> Option<(BigDecimal, Address)> {
        let a = self.token.amount_out(amount_in, self.direction().clone())?;
        let top_bytes = a.0.numer().to_bytes_le();
        let top = BigInt::from_bytes_le(top_bytes.0, &top_bytes.1);
    
        let bottom_bytes = a.0.denom().to_bytes_le();
        let bottom = BigInt::from_bytes_le(bottom_bytes.0, &bottom_bytes.1);
        let decimal = BigDecimal::from(top) / BigDecimal::from(bottom);
        Some((decimal,a.1))
    }

    pub fn has_enough_pending_reserve(&self, swap_reserve: U256) -> bool {
        let reserve = self.token.pending_reserve(self.direction().clone());
        if swap_reserve.gt(&reserve) {
            return false;
        }
        return true;
    }

    pub fn has_enough_confirmed_reserve(&self, swap_reserve: U256) -> bool {
        let reserve = self.token.confirmed_reserve(self.direction().clone());
        if swap_reserve.gt(&reserve) {
            return false;
        }
        return true;
    }

    pub fn router(&self) -> Address {
        self.token.pair.router
    }

    pub fn get_amount_in(amt_out: U256, reserve_in: U256, reserve_out: U256)->Option<U256> {
        CryptoPair::get_amount_in(amt_out, reserve_in, reserve_out)
    }

    pub fn get_amount_out(amt_in: U256, reserve_in: U256, reserve_out: U256)->Option<U256> {
        CryptoPair::get_amount_out(amt_in, reserve_in, reserve_out)
    }

    pub fn a_to_b(a1:U256, b1:U256, a2:U256, b2:U256)->Option<(U256,U256)> {
        CryptoPair::a_to_b(a1,b1,a2,b2)
    }

}
