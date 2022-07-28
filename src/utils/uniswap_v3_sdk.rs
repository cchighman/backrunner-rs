use std::ops::Div;
use std::str::FromStr;
use std::{
    cmp::Ordering,
    ops::{Mul, Rem},
};

use bigdecimal::BigDecimal;
use dashmap::DashMap;
use ethers::prelude::U256;
use lazy_static::lazy_static;
use math::round::ceil;
use math::round::floor;
use num_bigint::{BigInt, ToBigInt};
use num_rational::BigRational;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};

pub fn encode_price_sqrt(amount1: BigInt, amount0: BigInt) -> BigInt {
    let numerator = amount1 << 192;
    let ratio_x192: BigInt = numerator / amount0;
    ratio_x192.sqrt()
}

fn mul_shift(val: BigInt, mul_by: &[u8]) -> BigInt {
    let mul_by = BigInt::parse_bytes(mul_by, 16).unwrap();
    (val * mul_by) >> 128
}

pub fn sqrt_ratio_at_tick(tick: BigInt) -> BigInt {
    let min_tick: BigInt = -(887272.to_bigint().unwrap());

    let max_tick: BigInt = -(min_tick.clone());

    let max_uint_256: BigInt = BigInt::parse_bytes(
        b"115792089237316195423570985008687907853269984665640564039457584007913129639935",
        10,
    )
    .unwrap();

    let q32: BigInt = 2i32.to_bigint().unwrap().pow(32);

    assert!(tick >= min_tick && tick <= max_tick);
    let abs_tick = tick.abs();
    let mut ratio = if abs_tick.clone() & BigInt::parse_bytes(b"1", 16).unwrap() != BigInt::zero() {
        BigInt::parse_bytes(b"fffcb933bd6fad37aa2d162d1a594001", 16).unwrap()
    } else {
        BigInt::parse_bytes(b"100000000000000000000000000000000", 16).unwrap()
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"2", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"fff97272373d413259a46990580e213a")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"4", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"fff2e50f5f656932ef12357cf3c7fdcc")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"8", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"ffe5caca7e10e4e61c3624eaa0941cd0")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"10", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"ffcb9843d60f6159c9db58835c926644")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"20", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"ff973b41fa98c081472e6896dfb254c0")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"40", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"ff2ea16466c96a3843ec78b326b52861")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"80", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"fe5dee046a99a2a811c461f1969c3053")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"100", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"fcbe86c7900a88aedcffc83b479aa3a4")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"200", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"f987a7253ac413176f2b074cf7815e54")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"400", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"f3392b0822b70005940c7a398e4b70f3")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"800", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"e7159475a2c29b7443b29c7fa6e889d9")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"1000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"d097f3bdfd2022b8845ad8f792aa5825")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"2000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"a9f746462d870fdf8a65dc1f90e061e5")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"4000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"70d869a156d2a1b890bb3df62baf32f7")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"8000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"31be135f97d08fd981231505542fcfa6")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"10000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"9aa508b5b7a84e1c677de54f3e99bc9")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"20000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"5d6af8dedb81196699c329225ee604")
    } else {
        ratio
    };
    ratio = if (abs_tick.clone() & BigInt::parse_bytes(b"40000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"2216e584f5fa1ea926041bedfe98")
    } else {
        ratio
    };
    ratio = if (abs_tick & BigInt::parse_bytes(b"80000", 16).unwrap()) != BigInt::zero() {
        mul_shift(ratio, b"48a170391f7dc42444e8fa2")
    } else {
        ratio
    };

    ratio = if tick > BigInt::zero() {
        max_uint_256 / ratio
    } else {
        ratio
    };

    ratio = if ratio.clone().rem(q32.clone()) > BigInt::zero() {
        (ratio / q32) + BigInt::one()
    } else {
        ratio / q32
    };
    ratio
}

pub fn tick_at_sqrt_ratio(sqrt_ratio_x96: BigInt) -> i32 {
    let min_sqrt_ratio: BigInt = 4295128739i64.to_bigint().unwrap();

    let max_sqrt_ratio: BigInt = "1461446703485210103287273052203988822378723970342"
        .parse()
        .unwrap();

    assert!(sqrt_ratio_x96 >= min_sqrt_ratio && sqrt_ratio_x96 < max_sqrt_ratio);

    let sqrt_ratio_x128: BigInt = sqrt_ratio_x96.clone() << 32;
    let msb = most_significant_bit(sqrt_ratio_x128.clone());

    let mut r = if msb >= 128 {
        sqrt_ratio_x128 >> (msb - 127)
    } else {
        sqrt_ratio_x128 << (127 - msb)
    };

    let mut log_2: BigInt = (msb - 128.to_bigint().unwrap()) << 64;

    for i in 0..14 {
        r = (r.clone() * r) >> 127i32;
        let f: BigInt = r.clone() >> 128i32;
        log_2 |= f.clone() << (63 - i);
        r >>= f.clone().to_i64().unwrap();
    }

    let val: BigInt = "255738958999603826347141".parse().unwrap();
    let loq_sqrt0001: BigInt = log_2 * val;

    let val: BigInt = "3402992956809132418596140100660247210".parse().unwrap();
    let tick_low: BigInt = (loq_sqrt0001.clone() - val) >> 128;
    let tick_low = tick_low.to_i32().unwrap();

    let val: BigInt = "291339464771989622907027621153398088495".parse().unwrap();

    let tick_high: BigInt = (loq_sqrt0001 + val) >> 128;
    let tick_high = tick_high.to_i32().unwrap();

    if tick_low == tick_high {
        tick_low
    } else if sqrt_ratio_at_tick(tick_high.to_bigint().unwrap()) <= sqrt_ratio_x96 {
        tick_high
    } else {
        tick_low
    }
}

pub fn most_significant_bit(mut x: BigInt) -> u32 {
    assert!(x >= BigInt::zero());
    let two: BigInt = 2.to_bigint().unwrap();
    let powers_of_2: Vec<(u32, BigInt)> = [128u32, 64u32, 32u32, 16u32, 8u32, 4u32, 2u32, 1u32]
        .iter()
        .map(|x| (*x, two.pow(*x)))
        .collect();

    let max_uint_256: BigInt = BigInt::parse_bytes(
        b"115792089237316195423570985008687907853269984665640564039457584007913129639935",
        10,
    )
    .unwrap();
    assert!(x < max_uint_256);
    let mut msb = 0;

    for (power, min) in powers_of_2 {
        if x >= min {
            x >>= power as i32;
            msb += power;
        }
    }

    msb
}

#[derive(Clone, Debug)]
pub struct TokenDetails {
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    pub liquidity: String,
    pub sqrt_price: String,
    pub tick_spacing: String,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub symbol: String,
    pub address: String,
}

impl Token {
    pub fn sorts_before(&self, other: &Token) -> bool {
        self.address.to_lowercase() < other.address.to_lowercase()
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.symbol == other.symbol && self.address == other.address
    }
}

#[derive(Clone, Debug)]
pub struct Price {
    pub amount_0: BigInt,
    pub amount_1: BigInt,
    pub token_0: Token,
    pub token_1: Token,
}

impl Price {
    pub fn to_rational(&self) -> BigRational {
        BigRational::new(self.amount_0.clone(), self.amount_1.clone())
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.amount_0 == other.amount_0
            && self.amount_1 == other.amount_1
            && self.token_0 == other.token_0
            && self.token_1 == other.token_1
    }
}

impl Eq for Price {}

impl std::cmp::PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_rational().cmp(&other.to_rational())
    }
}

pub fn tick_to_price(base_token: Token, quote_token: Token, tick: BigInt) -> Price {
    let q96 = 2.to_bigint().unwrap().pow(96);
    let q192 = q96.pow(2);

    let sqrt_ratio_x96 = sqrt_ratio_at_tick(tick);
    let ratio_x192 = sqrt_ratio_x96.clone() * sqrt_ratio_x96;

    if base_token.sorts_before(&quote_token) {
        Price {
            token_0: base_token,
            token_1: quote_token,
            amount_0: q192,
            amount_1: ratio_x192,
        }
    } else {
        Price {
            token_0: base_token,
            token_1: quote_token,
            amount_0: ratio_x192,
            amount_1: q192,
        }
    }
}

pub fn price_to_tick(price: Price) -> i32 {
    let sorted = price.token_0.sorts_before(&price.token_1.clone());
    let sqrt_ratio_x96 = if sorted {
        encode_price_sqrt(price.amount_0.clone(), price.amount_1.clone())
    } else {
        encode_price_sqrt(price.amount_1.clone(), price.amount_0.clone())
    };

    let mut tick = tick_at_sqrt_ratio(sqrt_ratio_x96);

    let next_tick_price = tick_to_price(
        price.token_0.clone(),
        price.token_1.clone(),
        tick + BigInt::one(),
    );
    if sorted {
        if price >= next_tick_price {
            tick += 1;
        }
    } else if price <= next_tick_price {
        tick += 1;
    }
    tick
}

/// @notice Computes the amount of liquidity received for a given amount of token0 and price range
/// @dev Calculates amount0 * (sqrt(upper) * sqrt(lower)) / (sqrt(upper) - sqrt(lower))
/// @param sqrt_ratio_ax96 A sqrt price representing the first tick boundary
/// @param sqrt_ratio_bx96 A sqrt price representing the second tick boundary
/// @param amount0 The amount0 being sent in
/// @return liquidity The amount of returned liquidity
pub fn max_liquidity_for_amount0(
    sqrt_ratio_ax96: BigInt,
    sqrt_ratio_bx96: BigInt,
    amount0: i32,
) -> BigInt {
    let q96 = 2.to_bigint().unwrap().pow(96);
    let (sqrt_ratio_ax96, sqrt_ratio_bx96) = if sqrt_ratio_ax96 > sqrt_ratio_bx96 {
        (sqrt_ratio_bx96, sqrt_ratio_ax96)
    } else {
        (sqrt_ratio_ax96, sqrt_ratio_bx96)
    };

    let numerator = (amount0 * sqrt_ratio_ax96.clone()) * sqrt_ratio_bx96.clone();
    let denominator = (sqrt_ratio_bx96 - sqrt_ratio_ax96) * q96;
    numerator / denominator
}
/// @notice Computes the amount of liquidity received for a given amount of token1 and price range
/// @dev Calculates amount1 / (sqrt(upper) - sqrt(lower)).
/// @param sqrt_ratio_ax96 A sqrt price representing the first tick boundary
/// @param sqrt_ratio_bx96 A sqrt price representing the second tick boundary
/// @param amount1 The amount1 being sent in
/// @return liquidity The amount of returned liquidity
pub fn max_liquidity_for_amount1(
    sqrt_ratio_ax96: BigInt,
    sqrt_ratio_bx96: BigInt,
    amount1: i32,
) -> BigInt {
    let q96 = 2.to_bigint().unwrap().pow(96);
    let (sqrt_ratio_ax96, sqrt_ratio_bx96) = if sqrt_ratio_ax96 > sqrt_ratio_bx96 {
        (sqrt_ratio_bx96, sqrt_ratio_ax96)
    } else {
        (sqrt_ratio_ax96, sqrt_ratio_bx96)
    };
    (amount1 * q96) / (sqrt_ratio_bx96 - sqrt_ratio_ax96)
}

/// @notice Computes the maximum amount of liquidity received for a given amount of token0, token1, the current
/// pool prices and the prices at the tick boundaries
/// @param sqrtRatioX96 A sqrt price representing the current pool prices
/// @param sqrt_ratio_ax96 A sqrt price representing the first tick boundary
/// @param sqrt_ratio_bx96 A sqrt price representing the second tick boundary
/// @param amount0 The amount of token0 being sent in
/// @param amount1 The amount of token1 being sent in
/// @return liquidity The maximum amount of liquidity received
pub fn max_liquidity_for_amounts(
    sqrt_ratio_current_x96: BigInt,
    sqrt_ratio_ax96: BigInt,
    sqrt_ratio_bx96: BigInt,
    amount0: i32,
    amount1: i32,
) -> BigInt {
    let (sqrt_ratio_ax96, sqrt_ratio_bx96) = if sqrt_ratio_ax96 > sqrt_ratio_bx96 {
        (sqrt_ratio_bx96, sqrt_ratio_ax96)
    } else {
        (sqrt_ratio_ax96, sqrt_ratio_bx96)
    };
    if sqrt_ratio_current_x96 <= sqrt_ratio_ax96 {
        max_liquidity_for_amount0(sqrt_ratio_ax96, sqrt_ratio_bx96, amount0)
    } else if sqrt_ratio_current_x96 < sqrt_ratio_bx96 {
        let liquidity0 =
            max_liquidity_for_amount0(sqrt_ratio_current_x96.clone(), sqrt_ratio_bx96, amount0);
        let liquidity1 =
            max_liquidity_for_amount1(sqrt_ratio_ax96, sqrt_ratio_current_x96, amount1);
        if liquidity0 < liquidity1 {
            liquidity0
        } else {
            liquidity1
        }
    } else {
        max_liquidity_for_amount1(sqrt_ratio_ax96, sqrt_ratio_bx96, amount1)
    }
}

/**
 *  Computes the token0 and token1 value for a given amount of liquidity, the current
 *   pool prices and the prices at the tick boundaries
 *
 * @param  sqrt_ratio_x96 Current SQRT Price.
 * @param  sqrt_ratio_ax96 A sqrt price representing the first tick boundary.
 * @param  sqrt_ratio_bx96 A sqrt price representing the second tick boundary.
 * @param liquidityStr The liquidity being valued.
 * @return {Array<string>} A tuple with the reserves of token0 and token1.
 */
pub fn amounts_for_liquidity(
    sqrt_ratio_x96: &BigInt,
    sqrt_ratio_ax96: &BigInt,
    sqrt_ratio_bx96: &BigInt,
    liquidity: U256,
) -> [BigInt; 2] {
    let sqrt_ratio = sqrt_ratio_x96;
    let mut sqrt_ratio_a = sqrt_ratio_ax96;
    let mut sqrt_ratio_b = sqrt_ratio_bx96;

    if sqrt_ratio_a > sqrt_ratio_b {
        sqrt_ratio_a = sqrt_ratio_b;
        sqrt_ratio_b = sqrt_ratio_a;
    }

    let mut amount0 = BigInt::from(0);
    let mut amount1 = BigInt::from(0);

    if sqrt_ratio <= sqrt_ratio_a {
        amount0 = amount0for_liquidity(sqrt_ratio_a, sqrt_ratio_b, &liquidity);
    } else if sqrt_ratio < sqrt_ratio_b {
        amount0 = amount0for_liquidity(sqrt_ratio, sqrt_ratio_b, &liquidity);
        amount1 = amount1for_liquidity(sqrt_ratio_a, sqrt_ratio, &liquidity);
    } else {
        amount1 = amount1for_liquidity(sqrt_ratio_a, sqrt_ratio_b, &liquidity);
    }

    [amount0, amount1]
}

/// @notice Computes the amount of token0 for a given amount of liquidity and a price range
/// @param sqrt_ratio_ax96 A sqrt price representing the first tick boundary
/// @param sqrt_ratio_bx96 A sqrt price representing the second tick boundary
/// @param liquidity The liquidity being valued
/// @return amount0 The amount of token0
pub fn amount0for_liquidity(
    sqrt_ratio_ax96: &BigInt,
    sqrt_ratio_bx96: &BigInt,
    liquidity: &U256,
) -> BigInt {
    let mut sqrt_ratio_a = sqrt_ratio_ax96;
    let mut sqrt_ratio_b = sqrt_ratio_bx96;

    if sqrt_ratio_a > sqrt_ratio_b {
        sqrt_ratio_a = sqrt_ratio_b;
        sqrt_ratio_b = sqrt_ratio_a;
    }

    let left_shifted_liquidity = BigInt::from_str(
        (liquidity * U256::from(2).pow(U256::from(96)))
            .to_string()
            .as_str(),
    )
    .unwrap();
    let sqrt_diff = sqrt_ratio_b - sqrt_ratio_a.clone();
    let multiplied_res = left_shifted_liquidity * sqrt_diff;
    let numerator = multiplied_res / (sqrt_ratio_b.clone());

    

    numerator / sqrt_ratio_a.clone()
}

/**
 * Computes the amount of token1 for a given amount of liquidity and a price range.
 *
 * @param {bigint} sqrt_ratio_ax96 A sqrt price representing the first tick boundary.
 * @param {bigint} sqrt_ratio_bx96 A sqrt price representing the second tick boundary.
 * @param {bigint} liquidity The liquidity being valued.
 * @return {number} The amount of token1.
 */
pub fn amount1for_liquidity(
    sqrt_ratio_ax96: &BigInt,
    sqrt_ratio_bx96: &BigInt,
    liquidity: &U256,
) -> BigInt {
    let mut sqrt_ratio_a = sqrt_ratio_ax96;
    let mut sqrt_ratio_b = sqrt_ratio_bx96;

    if sqrt_ratio_a > sqrt_ratio_b {
        sqrt_ratio_a = sqrt_ratio_b;
        sqrt_ratio_b = sqrt_ratio_a;
    }

    let sqrt_diff = sqrt_ratio_b - sqrt_ratio_a.clone();
    let multiplied_res = BigInt::from_str(&*liquidity.to_string()).unwrap() * sqrt_diff;

    

    multiplied_res / BigInt::from(2).pow(96)
}

/**
 * Calculates the reserves of tokens based on the current tick value and formats
 * appropriately given the decimals of each token.
 *
 */
pub fn amounts_for_current_liquidity(
    token_decimals: Vec<u32>,
    liquidity_str: &String,
    sqrt_price_str: &String,
    tick_spacing: i32,
    tick_step: i32,
) -> [[BigInt; 2]; 2] {
    let tok0dec = BigInt::from(10).pow(token_decimals[0]);
    let tok1dec = BigInt::from(10).pow(token_decimals[1]);

    let sqrt_price = BigInt::from_str(sqrt_price_str).unwrap();
    let _sqrt_price_int = sqrt_price.to_bigint().unwrap();

    let _liquidity = BigInt::from_str(liquidity_str).unwrap();
    let liquidity_int = sqrt_price.to_bigint().unwrap();

    let tick = tick_at_sqrt_ratio(sqrt_price.clone());

    let [tick_low, tick_high] = tick_range(tick, tick_spacing, tick_step);

    let sqrt_a = sqrt_ratio_at_tick(tick_low.to_bigint().unwrap());
    let sqrt_b = sqrt_ratio_at_tick(tick_high.to_bigint().unwrap());
    // Calculate liquidity for both tokens
    let reserves = amounts_for_liquidity(
        &sqrt_price,
        &sqrt_a,
        &sqrt_b,
        U256::from_dec_str(liquidity_int.to_string().as_str()).unwrap(),
    );

    let [token0raw_liquidity, token1raw_liquidity] = reserves;

    let fraction0 = [token0raw_liquidity, tok0dec];
    let fraction1 = [token1raw_liquidity, tok1dec];

    dbg!("high - {}", tick_high);
    dbg!("low - {}", tick_low);
    [fraction0, fraction1]
}

pub fn tick_range(tick: i32, tick_spacing: i32, tick_step: i32) -> [BigInt; 2] {
    let tick_spacing_stepped = tick_spacing * tick_step;

    let ratio = floor(f64::from(tick / tick_spacing), 0) as i32;
    let calc = ratio * tick_spacing - tick_spacing_stepped;

    let tick_low = BigInt::from_i32(calc).unwrap();
    let tick_high = tick_low.clone() + BigInt::from(tick_spacing) + BigInt::from(tick_spacing_stepped * 2);

    [tick_low, tick_high]
}

pub enum FeeAmount {
    LOW = 500,
    MEDIUM = 3000,
    HIGH = 10000,
}
lazy_static! {
    pub static ref TICK_SPACING: DashMap<i32, i32> = {
        let m = DashMap::new();
        m.insert(500, 10);
        m.insert(3000, 60);
        m.insert(10000, 200);
        m
    };
}

pub fn min_tick(tick_spacing: i32) -> i32 {
    ceil(-887272_f64 / tick_spacing as f64, 0)
        .to_i32()
        .unwrap()
        * tick_spacing
}

pub fn max_tick(tick_spacing: i32) -> i32 {
    floor(887272_f64 / tick_spacing as f64, 0)
        .to_i32()
        .unwrap()
        * tick_spacing
}

pub fn max_liquidity_per_tick(tick_spacing: i32) -> i128 {
    let numerator = i128::from(2).pow(128) - i128::from(1);
    let denominator =
        (max_tick(tick_spacing as i32) - min_tick(tick_spacing as i32)) / tick_spacing + 1;
    numerator / denominator as i128
}

pub fn apply_sqrt_ratio_bips_hundredths_delta(
    sqrt_ratio: &BigDecimal,
    bips_hundredths: i32,
) -> BigDecimal {
    return BigDecimal::from_str(
        floor(
            sqrt_ratio
                .mul(sqrt_ratio)
                .mul(
                    BigDecimal::from_str((1e6 + bips_hundredths as f64).to_string().as_str())
                        .unwrap(),
                )
                .div(BigDecimal::from_str(1e6.to_string().as_str()).unwrap())
                .to_f64()
                .unwrap(),
            0,
        )
        .to_string()
        .as_str(),
    )
    .unwrap()
    .sqrt()
    .unwrap();
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use num_bigint::ToBigInt;
    use num_rational::BigRational;

    use super::*;

    #[test]
    fn encode_price_sqrt_1() {
        let x = encode_price_sqrt(100.to_bigint().unwrap(), 1.to_bigint().unwrap());
        assert_eq!(x, 792281625142643375935439503360i128.to_bigint().unwrap());
    }

    #[test]
    fn encode_price_sqrt_2() {
        let x = encode_price_sqrt(1.to_bigint().unwrap(), 100.to_bigint().unwrap());
        assert_eq!(x, 7922816251426433759354395033i128.to_bigint().unwrap());
    }

    #[test]
    fn encode_price_sqrt_3() {
        let x = encode_price_sqrt(111.to_bigint().unwrap(), 333.to_bigint().unwrap());
        assert_eq!(x, 45742400955009932534161870629i128.to_bigint().unwrap());
    }

    #[test]
    fn encode_price_sqrt_4() {
        let x = encode_price_sqrt(333.to_bigint().unwrap(), 111.to_bigint().unwrap());
        assert_eq!(x, 137227202865029797602485611888i128.to_bigint().unwrap());
    }

    #[test]
    fn sqrt_ratio_at_tick_1() {
        let min_tick = -887272i32;

        let min_sqrt_ratio: BigInt = 4295128739i64.to_bigint().unwrap();

        let x = sqrt_ratio_at_tick(min_tick.to_bigint().unwrap());
        assert_eq!(x, min_sqrt_ratio);
    }

    #[test]
    fn sqrt_ratio_at_tick_2() {
        let min_tick = -887272i32;
        let max_tick = -min_tick;

        let max_sqrt_ratio: BigInt =
            BigInt::parse_bytes(b"1461446703485210103287273052203988822378723970342", 10).unwrap();

        let x = sqrt_ratio_at_tick(max_tick.to_bigint().unwrap());
        assert_eq!(x, max_sqrt_ratio);
    }

    #[test]
    fn tick_at_sqrt_ratio_1() {
        let min_tick = -887272i32;
        let _max_tick = -min_tick;

        let min_sqrt_ratio: BigInt = 4295128739i64.to_bigint().unwrap();

        let x = tick_at_sqrt_ratio(min_sqrt_ratio);
        assert_eq!(x, min_tick);
    }

    #[test]
    fn tick_at_sqrt_ratio_2() {
        let min_tick = -887272i32;
        let max_tick = -min_tick;

        let max_sqrt_ratio: BigInt =
            BigInt::parse_bytes(b"1461446703485210103287273052203988822378723970342", 10).unwrap();

        let x = tick_at_sqrt_ratio(max_sqrt_ratio - BigInt::one());
        assert_eq!(x, max_tick - 1);
    }

    #[test]
    fn test_tick_range() {
        // should return the expected tick range
        let [range_low, range_high] = tick_range(1001, 60, 0);
        assert_eq!(range_low, BigInt::from(960));
        assert_eq!(range_high, BigInt::from(1020));
    }

    #[test]
    fn test_tick_range_wfn() {
        let [range_low, range_high] = tick_range(1001, 60, 5);
        assert_eq!(range_low, BigInt::from(660));
        assert_eq!(range_high, BigInt::from(1320));
    }

    #[test]
    fn test_most_significant_bfn() {
        let two: BigInt = 2.to_bigint().unwrap();

        for i in 1u32..256u32 {
            let x = two.pow(i);
            assert_eq!(i, most_significant_bit(x))
        }

        for i in 2u32..256u32 {
            let x = two.pow(i) - BigInt::one();
            assert_eq!(i - 1, most_significant_bit(x))
        }
    }

    #[test]
    fn test_ticks_to_price() {
        let t0 = Token {
            symbol: "TestToken0".to_string(),
            address: "0x1".to_string(),
        };
        let t1 = Token {
            symbol: "TestToken1".to_string(),
            address: "0x0".to_string(),
        };
        let price = tick_to_price(t0, t1, -(276225.to_bigint().unwrap()));

        let scalar = BigRational::new(
            10.to_bigint().unwrap().pow(18),
            10.to_bigint().unwrap().pow(6),
        );

        let price_rational = price.to_rational() * scalar;

        assert_eq!(
            price_rational.to_f64().unwrap().to_string(),
            "1.0099513373596989"
        )
    }

    #[test]
    fn test_ticks_to_price_2() {
        let t0 = Token {
            symbol: "TestToken0".to_string(),
            address: "0x1".to_string(),
        };
        let t1 = Token {
            symbol: "TestToken1".to_string(),
            address: "0x0".to_string(),
        };
        let price = tick_to_price(t0, t1, -(276423.to_bigint().unwrap()));

        let scalar = BigRational::new(
            10.to_bigint().unwrap().pow(18),
            10.to_bigint().unwrap().pow(6),
        );

        let price_rational = price.to_rational() * scalar;

        assert_eq!(
            price_rational.to_f64().unwrap().to_string(),
            "0.990151951561538"
        )
    }

    #[test]
    fn test_price_to_ticks() {
        let t0 = Token {
            symbol: "TestToken0".to_string(),
            address: "0x1".to_string(),
        };
        let t1 = Token {
            symbol: "TestToken1".to_string(),
            address: "0x0".to_string(),
        };

        let price = Price {
            amount_0: 100e18.to_bigint().unwrap(),
            amount_1: 101e6.to_bigint().unwrap(),
            token_0: t0.clone(),
            token_1: t1.clone(),
        };
        let tick = price_to_tick(price);
        assert_eq!(tick, -276225);

        let price = Price {
            amount_0: 1.to_bigint().unwrap(),
            amount_1: "1800".parse().unwrap(),
            token_0: t1.clone(),
            token_1: t0.clone(),
        };
        let tick = price_to_tick(price);
        assert_eq!(tick, -74960);

        let price = Price {
            amount_0: 100e18.to_bigint().unwrap(),
            amount_1: 101e6.to_bigint().unwrap(),
            token_0: t0,
            token_1: t1,
        };
        let tick = price_to_tick(price);
        assert_eq!(tick, -276225);
    }

    #[test]
    fn test_max_liquidity_for_amounts0() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(1), BigInt::from(1));
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let liquidity =
            max_liquidity_for_amounts(sqrt_price_x96, sqrt_price_ax96, sqrt_price_bx96, 100, 200);
        assert_eq!(liquidity, BigInt::from(2148));
    }

    fn test_liquidity_below_price() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(99), BigInt::from(100));
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let liquidity =
            max_liquidity_for_amounts(sqrt_price_x96, sqrt_price_ax96, sqrt_price_bx96, 100, 200);
        assert_eq!(liquidity, BigInt::from(1048));
    }

    fn test_liquidity_above_price() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(111), BigInt::from(100));
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));

        let liquidity =
            max_liquidity_for_amounts(sqrt_price_x96, sqrt_price_ax96, sqrt_price_bx96, 100, 200);
        assert_eq!(liquidity, BigInt::from(2097));
    }

    fn test_liquidity_equal_lower_price() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(100), BigInt::from(10));
        let sqrt_price_ax96 = sqrt_price_x96.clone();
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));

        let liquidity = max_liquidity_for_amounts(
            sqrt_price_x96,
            sqrt_price_ax96,
            sqrt_price_bx96,
            100,
            200,
        );
        assert_eq!(liquidity, BigInt::from(1048));
    }

    fn test_liquidity_equal_above_price() {
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let sqrt_price_x96 = sqrt_price_bx96.clone();

        let liquidity = max_liquidity_for_amounts(
            sqrt_price_x96,
            sqrt_price_ax96,
            sqrt_price_bx96,
            100,
            200,
        );
        assert_eq!(liquidity, BigInt::from(2097));
    }

    fn test_amounts_for_price_inside() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(1), BigInt::from(1));
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let [amount0, amount1] = amounts_for_liquidity(
            &sqrt_price_x96,
            &sqrt_price_ax96,
            &sqrt_price_bx96,
            U256::from(2148),
        );
        assert_eq!(amount0, BigInt::from(99));
        assert_eq!(amount1, BigInt::from(99));
    }

    fn test_amount_for_price_below() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(99), BigInt::from(100));
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let [amount0, amount1] = amounts_for_liquidity(
            &sqrt_price_x96,
            &sqrt_price_ax96,
            &sqrt_price_bx96,
            U256::from(1048),
        );
        assert_eq!(amount0, BigInt::from(99));
        assert_eq!(amount1, BigInt::from(0));
    }

    fn test_amount_price_lower_boundary() {
        let sqrt_price_x96 = encode_price_sqrt(BigInt::from(111), BigInt::from(100));
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let [amount0, amount1] = amounts_for_liquidity(
            &sqrt_price_x96,
            &sqrt_price_ax96,
            &sqrt_price_bx96,
            U256::from(2097),
        );
        assert_eq!(amount0, BigInt::from(99));
        assert_eq!(amount1, BigInt::from(0));
    }

    fn test_amounts_price_upper_boundary() {
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_x96 = sqrt_price_ax96.clone();
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let [amount0, amount1] = amounts_for_liquidity(
            &sqrt_price_x96,
            &sqrt_price_ax96,
            &sqrt_price_bx96,
            U256::from(1048),
        );
        assert_eq!(amount0, BigInt::from(0));
        assert_eq!(amount1, BigInt::from(199));
    }

    fn test_amounts_above_price() {
        let sqrt_price_ax96 = encode_price_sqrt(BigInt::from(100), BigInt::from(110));
        let sqrt_price_bx96 = encode_price_sqrt(BigInt::from(110), BigInt::from(100));
        let sqrt_price_x96 = sqrt_price_bx96.clone();
        let [amount0, amount1] = amounts_for_liquidity(
            &sqrt_price_x96,
            &sqrt_price_ax96,
            &sqrt_price_bx96,
            U256::from(2097),
        );
        assert_eq!(amount0, BigInt::from(0));
        assert_eq!(amount1, BigInt::from(199));
    }
}
