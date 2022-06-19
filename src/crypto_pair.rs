use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;


use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use ethers::prelude::{Address, U256};
use futures_signals::signal::{Mutable, MutableSignal};
use num_bigint::BigInt;
use num_rational::{Ratio, BigRational};
use serde::{Deserialize, Serialize};
use ethers::core::types::transaction::eip2718::TypedTransaction;

use crate::dex_pool::DexPool;
use crate::utils::common::DIRECTION;
use crate::utils::conversions::*;
use crate::utils::u256_decimal::*;
use crate::utils::ratio_as_decimal::*;
use crate::utils::conversions::U256Ext;




const POOL_SWAP_GAS_COST: usize = 60_000;

lazy_static::lazy_static! {
    static ref POOL_MAX_RESERVES: U256 = U256::from((1u128 << 112) - 1);
}

/// This type denotes `(reserve_a, reserve_b, token_b)` where
/// `reserve_a` refers to the reserve of the excluded token.
type RelativeReserves = (U256, U256, Address);


 // Some ERC20s (e.g. AMPL) have an elastic supply and can thus reduce the balance of their owners without any transfer or other interaction ("rebase").
        // Such behavior can implicitly change the *k* in the pool's constant product formula. E.g. a pool with 10 USDC and 10 AMPL has k = 100. After a negative
        // rebase the pool's AMPL balance may reduce to 9, thus k should be implicitly updated to 90 (figuratively speaking the pool is undercollateralized).
        // Uniswap pools however only update their reserves upon swaps. Such an "out of sync" pool has numerical issues when computing the right clearing price.
        // Note, that a positive rebase is not problematic as k would increase in this case giving the pool excess in the elastic token (an arbitrageur could
        // benefit by withdrawing the excess from the pool without selling anything).
        // We therefore exclude all pools where the pool's token balance of either token in the pair is less than the cached reserve.

    
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPair {
    pub(crate) pair: DexPool,
    pub(crate) pending_left_reserves: Mutable<ethers::prelude::U256>,
    pub(crate) pending_right_reserves: Mutable<ethers::prelude::U256>,
    pub(crate) confirmed_left_reserves: Mutable<ethers::prelude::U256>,
    pub(crate) confirmed_right_reserves: Mutable<ethers::prelude::U256>,
    pub(crate) pending_txs: Mutable<Vec<TypedTransaction>>,
    pub(crate) fee: Ratio<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPairs {
    pub pairs: Vec<Vec<CryptoPair>>,
}


impl CryptoPair {
    /* TODO - ID uses pool id, mempool event uses id to trigger an update method.
        Update method updates reserves and then parallel iterates path references to
        invoke recalculate.
    */
    pub fn new(pair: DexPool) -> Self {
        Self {
            pair: pair.clone(),
            pending_left_reserves: Mutable::new(pair.token0.reserve.clone()),
            pending_right_reserves: Mutable::new(pair.token1.reserve.clone()),
            confirmed_left_reserves: Mutable::new(pair.token0.reserve.clone()),
            confirmed_right_reserves: Mutable::new(pair.token1.reserve.clone()),
            pending_txs:  Default::default(),
            fee: Ratio::new(3, 1000)
        }
    }

    pub fn left_symbol(&self) -> &String {
        return &self.pair.token0.symbol;
    }

    pub fn right_symbol(&self) -> &String {
        return &self.pair.token1.symbol;
    }

    pub fn left_decimal(&self) -> i32 {
        return self.pair.token0.decimals;
    }
    pub fn symbol(&self, direction: DIRECTION) -> &String {
        if direction == DIRECTION::Left {
            return &self.pair.token0.symbol;
        } else {
            return &self.pair.token1.symbol;
        }
    }

    pub fn right_decimal(&self) -> i32 {
        return self.pair.token1.decimals;
    }

    pub fn decimal(&self, direction: DIRECTION) -> i32 {
        if direction == DIRECTION::Left {
            return self.pair.token0.decimals;
        } else {
            return self.pair.token1.decimals;
        }
    }

    pub fn other_decimal(&self, direction: DIRECTION) -> i32 {
        if direction == DIRECTION::Left {
            return self.pair.token1.decimals;
        } else {
            return self.pair.token0.decimals;
        }
    }

    pub fn confirmed_left_reserves(&self) -> U256 {
        return self.confirmed_left_reserves.get();
    }

    pub fn confirmed_right_reserves(&self) -> U256 {
        return self.confirmed_right_reserves.get();
    }

    pub fn pending_left_reserves(&self) -> U256 {
        return self.pending_left_reserves.get();
    }

    pub fn pending_right_reserves(&self) -> U256 {
        return self.pending_right_reserves.get();
    }

    pub fn pair_id(&self) -> &Address {
        return &self.pair.id;
    }

    pub fn dex(&self) -> &String {
        return &self.pair.dex;
    }

    pub fn router(&self) -> Address {
        return self.pair.router;
    }

    pub fn left_id(&self) -> &Address {
        return &self.pair.token0.id;
    }

    pub fn right_id(&self) -> &Address {
        return &self.pair.token1.id;
    }

    pub fn id(&self, direction: DIRECTION) -> &Address {
        if direction == DIRECTION::Left {
            return self.left_id();
        } else {
            return self.right_id();
        }
    }

    pub fn pair_symbol(&self) -> String {
        return self.left_symbol().to_owned() + self.right_symbol();
    }

    pub fn update(&self, new: &U256) {
        self.pending_left_reserves.set(&self.pending_left_reserves.get() + new);
        
    }

    pub fn pending_left_reserves_signal(&self) -> MutableSignal<U256> {
        self.pending_left_reserves.signal()
    }


    pub fn pending_right_reserves_signal(&self) -> MutableSignal<U256> {
        self.pending_right_reserves.signal()
    }

    pub fn confirmed_left_reserves_signal(&self) -> MutableSignal<U256> {
        self.confirmed_left_reserves.signal()
    }


    pub fn confirmed_right_reserves_signal(&self) -> MutableSignal<U256> {
        self.confirmed_right_reserves.signal()
    }


    pub fn pending_reserves_signal(&self, direction: DIRECTION) -> MutableSignal<U256> {
        if direction == DIRECTION::Left {
            return self.pending_left_reserves_signal();
        } else {
            return self.pending_right_reserves_signal();
        }
    }

    pub fn confirmed_reserves_signal(&self, direction: DIRECTION) -> MutableSignal<U256> {
        if direction == DIRECTION::Left {
            return self.confirmed_left_reserves_signal();
        } else {
            return self.confirmed_right_reserves_signal();
        }
    }

    pub fn pending_reserve(&self, direction: DIRECTION) -> U256 {
        if direction == DIRECTION::Left {
            return self.pending_left_reserves();
        } else {
            return self.pending_right_reserves();
        }
    }

    pub fn confirmed_reserve(&self, direction: DIRECTION) -> U256 {
        if direction == DIRECTION::Left {
            return self.confirmed_left_reserves();
        } else {
            return self.confirmed_right_reserves();
        }
    }

    pub fn reserve(&self, direction: DIRECTION) -> U256 {
        if direction == DIRECTION::Left {
            return self.confirmed_left_reserves();
        } else {
            return self.confirmed_right_reserves();
        }
    }

    pub fn pending_signal(&self, direction: DIRECTION) -> MutableSignal<U256> {
        if direction == DIRECTION::Left {
            return self.pending_left_reserves_signal();
        } else {
            return self.pending_right_reserves_signal();
        }
    }

    pub fn confirmed_signal(&self, direction: DIRECTION) -> MutableSignal<U256> {
        if direction == DIRECTION::Left {
            return self.confirmed_left_reserves_signal();
        } else {
            return self.confirmed_right_reserves_signal();
        }
    }
    
    pub fn pending_txs(&self)->Vec<TypedTransaction> {
        self.pending_txs.get_cloned()
    }


    /* Reasoning in terms of price exclusively uses reserves */
    pub fn rational_price(&self, direction: DIRECTION)->Result<Ratio<BigInt>, anyhow::Error> {
        if direction == DIRECTION::Left {
            BigRational::new_checked(self.pending_right_reserves.get().to_big_int(), self.pending_left_reserves.get().to_big_int())
        } else {
            BigRational::new_checked(self.pending_left_reserves.get().to_big_int(), self.pending_right_reserves.get().to_big_int())
        }
    }

    pub fn decimal_price(&self, direction: DIRECTION)->Result<BigDecimal, anyhow::Error> {
        let rational_price =  self.rational_price(direction)?;
        let top_bytes = rational_price.numer().to_bytes_le();
        let top = BigInt::from_bytes_le(top_bytes.0, &top_bytes.1);
    
        let bottom_bytes = rational_price.denom().to_bytes_le();
        let bottom = BigInt::from_bytes_le(bottom_bytes.0, &bottom_bytes.1);

        Ok(BigDecimal::from(top) / BigDecimal::from(bottom))
    }
    
    /* Reasoning in terms of a relative token amount requires consideration of decimal placement */
    pub fn to_amount(&self, reserve_amt: &U256, direction: DIRECTION)->Result<BigDecimal, anyhow::Error> {
        let rational_price = self.rational_price(direction.clone()).unwrap().mul(BigInt::from_i128(10_i128.pow((self.decimal(direction.clone()) - self.other_decimal(direction.clone()).abs()).try_into().unwrap())).unwrap());
        let top_bytes = rational_price.numer().to_bytes_le();
        let top = BigInt::from_bytes_le(top_bytes.0, &top_bytes.1);
    
        let bottom_bytes = rational_price.denom().to_bytes_le();
        let bottom = BigInt::from_bytes_le(bottom_bytes.0, &bottom_bytes.1);

        Ok(BigDecimal::from(top) / BigDecimal::from(bottom))
    }    
 
    /* Reasoning in terms of a relative token amount requires consideration of decimal placement */
    pub fn to_reserve(&self, token_amt: &BigDecimal, direction: DIRECTION)->Result<U256, anyhow::Error> {
        let (x, exp) = token_amt.clone().into_bigint_and_exponent();
        let numerator_bytes = x.to_bytes_le();
        let base = BigInt::from_bytes_le(numerator_bytes.0, &numerator_bytes.1);
        let ten = BigRational::new(10.into(), 1.into());
        let numerator = BigRational::new(base, 1.into());
        let added_exp = ten.pow((exp.add(self.decimal(direction) as i64)) as i32);
        big_rational_to_u256(&(numerator/added_exp))
    }    

    /// Given an input amount and token, returns the maximum output amount and address of the other asset.
    /// Returns None if operation not possible due to arithmetic issues (e.g. over or underflow)
    pub fn amount_out(&self, token_in: Address, amount_in: U256) -> Option<(U256, Address)> {
        let (reserve_in, reserve_out, token_out) = self.relative_reserves(token_in);
        Some((
            self.amount_out_impl(amount_in, reserve_in, reserve_out)?,
            token_out,
        ))
    }

    /// Given an output amount and token, returns a required input amount and address of the other asset.
    /// Returns None if operation not possible due to arithmetic issues (e.g. over or underflow, reserve too small)
    pub fn amount_in(&self, token_out: Address, amount_out: U256) -> Option<(U256, Address)> {
        let (reserve_out, reserve_in, token_in) = self.relative_reserves(token_out);
        Some((
            self.amount_in_impl(amount_out, reserve_in, reserve_out)?,
            token_in,
        ))
    }
  /// Given one of the pool's two tokens, returns a tuple containing the `RelativeReserves`
    /// along with the opposite token. That is, the elements returned are (respectively)
    /// - the pool's reserve of token provided
    /// - the reserve of the other token
    /// - the pool's other token
    /// This is essentially a helper method for shuffling values in `amount_in` and `amount_out`
    fn relative_reserves(&self, token: Address) -> RelativeReserves {
        // https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol#L53
        if token == *self.left_id() {
            (
                self.confirmed_left_reserves(),
                self.confirmed_right_reserves(),
                *self.right_id(),
            )
        } else {
            assert_eq!(token, *self.right_id(), "Token not part of pool");
            (
                self.confirmed_right_reserves(),
                self.confirmed_left_reserves(),
                *self.left_id(),
            )
        }
    }

    fn amount_out_impl(&self, amount_in: U256, reserve_in: U256, reserve_out: U256) -> Option<U256> {
        if amount_in.is_zero() || reserve_in.is_zero() || reserve_out.is_zero() {
            return None;
        }

        let amount_in_with_fee =
            amount_in.checked_mul(U256::from(self.fee.denom().checked_sub(*self.fee.numer())?))?;
        let numerator = amount_in_with_fee.checked_mul(reserve_out)?;
        let denominator = reserve_in
            .checked_mul(U256::from(*self.fee.denom()))?
            .checked_add(amount_in_with_fee)?;
        let amount_out = numerator.checked_div(denominator)?;

        check_final_reserves(amount_in, amount_out, reserve_in, reserve_out)?;
        Some(amount_out)
    }

    fn amount_in_impl(&self, amount_out: U256, reserve_in: U256, reserve_out: U256) -> Option<U256> {
        if amount_out.is_zero() || reserve_in.is_zero() || reserve_out.is_zero() {
            return None;
        }

        let numerator = reserve_in
            .checked_mul(amount_out)?
            .checked_mul(U256::from(*self.fee.denom()))?;
        let denominator = reserve_out
            .checked_sub(amount_out)?
            .checked_mul(U256::from(self.fee.denom().checked_sub(*self.fee.numer())?))?;
        let amount_in = numerator.checked_div(denominator)?.checked_add(1.into())?;

        check_final_reserves(amount_in, amount_out, reserve_in, reserve_out)?;
        Some(amount_in)
    }
}
    /* 
///
/// Given information about the shortage token (the one we need to take from Uniswap) and the excess token (the one we give to Uniswap), this function
/// computes the exact out_amount required from Uniswap to perfectly match demand and supply at the effective Uniswap price (the one used for that in/out swap).
///
/// The derivation of this formula is described in https://docs.google.com/document/d/1jS22wxbCqo88fGsqEMZgRQgiAcHlPqxoMw3CJTHst6c/edit
/// It assumes GP fee (φ) to be 1
///
fn compute_uniswap_out(
    shortage: &TokenContext,
    excess: &TokenContext,
    amm_fee: Ratio<u32>,
) -> Option<BigRational> {
    let numerator_minuend = (amm_fee.denom() - amm_fee.numer())
        * (u256_to_big_int(&excess.sell_volume) - u256_to_big_int(&excess.buy_volume))
        * u256_to_big_int(&shortage.reserve);
    let numerator_subtrahend = amm_fee.denom()
        * (u256_to_big_int(&shortage.sell_volume) - u256_to_big_int(&shortage.buy_volume))
        * u256_to_big_int(&excess.reserve);
    let denominator: BigInt = amm_fee.denom() * u256_to_big_int(&excess.reserve)
        + (amm_fee.denom() - amm_fee.numer())
            * (u256_to_big_int(&excess.sell_volume) - u256_to_big_int(&excess.buy_volume));
    BigRational::new_checked(numerator_minuend - numerator_subtrahend, denominator).ok()
}

///
/// Given the desired amount to receive and the state of the pool, this computes the required amount
/// of tokens to be sent to the pool.
/// Taken from: https://github.com/Uniswap/uniswap-v2-periphery/blob/4123f93278b60bcf617130629c69d4016f9e7584/contracts/libraries/UniswapV2Library.sol#L53
/// Not adding + 1 in the end, because we are working with rationals and thus don't round up.
///
fn compute_uniswap_in(
    out: BigRational,
    shortage: &TokenContext,
    excess: &TokenContext,
    amm_fee: Ratio<u32>,
) -> Option<BigRational> {
    let numerator = U256::from(*amm_fee.denom()).to_big_rational()
        * out.clone()
        * u256_to_big_int(&excess.reserve);
    let denominator = U256::from(amm_fee.denom() - amm_fee.numer()).to_big_rational()
        * (shortage.reserve.to_big_rational() - out);
    numerator.checked_div(&denominator)
}
}
*/
    
fn check_final_reserves(
    amount_in: U256,
    amount_out: U256,
    reserve_in: U256,
    reserve_out: U256,
) -> Option<(U256, U256)> {
    let final_reserve_in = reserve_in.checked_add(amount_in)?;
    let final_reserve_out = reserve_out.checked_sub(amount_out)?;

    if final_reserve_in > *POOL_MAX_RESERVES {
        None
    } else {
        Some((final_reserve_in, final_reserve_out))
    }
}

pub struct PairUpdateParams {}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
impl fmt::Display for CryptoPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ethcontract_error;

    #[test]
    fn test_amounts_out() {
        let sell_token = H160::from_low_u64_be(1);
        let buy_token = H160::from_low_u64_be(2);

        // Even Pool
        let pool = Pool::uniswap(TokenPair::new(sell_token, buy_token).unwrap(), (100, 100));
        assert_eq!(
            pool.amount_out(sell_token, 10.into()),
            Some((9.into(), buy_token))
        );
        assert_eq!(
            pool.amount_out(sell_token, 100.into()),
            Some((49.into(), buy_token))
        );
        assert_eq!(
            pool.amount_out(sell_token, 1000.into()),
            Some((90.into(), buy_token))
        );

        //Uneven Pool
        let pool = Pool::uniswap(TokenPair::new(sell_token, buy_token).unwrap(), (200, 50));
        assert_eq!(
            pool.amount_out(sell_token, 10.into()),
            Some((2.into(), buy_token))
        );
        assert_eq!(
            pool.amount_out(sell_token, 100.into()),
            Some((16.into(), buy_token))
        );
        assert_eq!(
            pool.amount_out(sell_token, 1000.into()),
            Some((41.into(), buy_token))
        );

        // Large Numbers
        let pool = Pool::uniswap(
            TokenPair::new(sell_token, buy_token).unwrap(),
            (1u128 << 90, 1u128 << 90),
        );
        assert_eq!(
            pool.amount_out(sell_token, 10u128.pow(20).into()),
            Some((99_699_991_970_459_889_807u128.into(), buy_token))
        );

        // Overflow
        assert_eq!(pool.amount_out(sell_token, U256::max_value()), None);
    }

    #[test]
    fn test_amounts_in() {
        let sell_token = H160::from_low_u64_be(1);
        let buy_token = H160::from_low_u64_be(2);

        // Even Pool
        let pool = Pool::uniswap(TokenPair::new(sell_token, buy_token).unwrap(), (100, 100));
        assert_eq!(
            pool.amount_in(buy_token, 10.into()),
            Some((12.into(), sell_token))
        );
        assert_eq!(
            pool.amount_in(buy_token, 99.into()),
            Some((9930.into(), sell_token))
        );

        // Buying more than possible
        assert_eq!(pool.amount_in(buy_token, 100.into()), None);
        assert_eq!(pool.amount_in(buy_token, 1000.into()), None);

        //Uneven Pool
        let pool = Pool::uniswap(TokenPair::new(sell_token, buy_token).unwrap(), (200, 50));
        assert_eq!(
            pool.amount_in(buy_token, 10.into()),
            Some((51.into(), sell_token))
        );
        assert_eq!(
            pool.amount_in(buy_token, 49.into()),
            Some((9830.into(), sell_token))
        );

        // Large Numbers
        let pool = Pool::uniswap(
            TokenPair::new(sell_token, buy_token).unwrap(),
            (1u128 << 90, 1u128 << 90),
        );
        assert_eq!(
            pool.amount_in(buy_token, 10u128.pow(20).into()),
            Some((100_300_910_810_367_424_267u128.into(), sell_token)),
        );
    }

    #[test]
    fn computes_final_reserves() {
        assert_eq!(
            check_final_reserves(1.into(), 2.into(), 1_000_000.into(), 2_000_000.into(),).unwrap(),
            (1_000_001.into(), 1_999_998.into()),
        );
    }

    #[test]
    fn check_final_reserve_limits() {
        // final out reserve too low
        assert!(check_final_reserves(0.into(), 1.into(), 1_000_000.into(), 0.into()).is_none());
        // final in reserve too high
        assert!(
            check_final_reserves(1.into(), 0.into(), *POOL_MAX_RESERVES, 1_000_000.into())
                .is_none()
        );
    }

    */