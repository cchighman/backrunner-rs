use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use ethereum_types::{U256, U512};
use num_traits::real::Real;
use num_traits::{Pow, ToPrimitive};

use crate::call_julia::route_cfmms;
use crate::crypto_pair::CryptoPair;

// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
pub fn get_amount_out(amountIn: U512, reserveIn: U512, reserveOut: U512) -> U512 {
    let amountInWithFee = amountIn.mul(U512::from(997));
    let numerator: U512 = amountInWithFee.mul(reserveOut);
    let denominator: U512 = reserveIn.mul(U512::from(1000)).add(amountInWithFee);
    /*
    println!(
        "get_amount_out - numerator: {}  denominator: {}  ratio: {}",
        numerator,
        denominator,
        numerator / denominator
    );
    */
    return numerator / denominator;
}

// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
pub fn get_amount_in(amountOut: U512, reserveIn: U512, reserveOut: U512) -> U512 {
    let numerator = reserveIn * amountOut * 1000;
    /*
        println!(
            "get_amount_in - amountOut: {}  reserveIn: {}  reserveOut: {}",
            amountOut, reserveIn, reserveOut
        );
    */
    let mut denominator = reserveOut.checked_sub(amountOut);
    if denominator == None {
        return U512::one();
    }
    let denom: U512 = denominator.unwrap().mul(997);
    /*
    println!(
        "get_amount_in - numerator: {}  denominator: {}  ratio: {}",
        numerator,
        denom,
        numerator / denom
    );
    */

    return (numerator / denom);
}

pub fn get_amounts_out(amountIn: U512, paths: &Vec<Arc<CryptoPair>>) -> Vec<U512> {
    let mut amounts = Vec::new();

    amounts.push(amountIn);
    for i in 0..paths.len() {
        /*
        println!(
            "get_amounts_out - pair - left: {}  right: {}",
            paths[i].left_id(),
            paths[i].right_id()
        );
        */
        amounts.push(get_amount_out(
            amounts[i],
            U512::from_dec_str(&*paths[i].left_reserves().to_string()).unwrap(),
            U512::from_dec_str(&*paths[i].right_reserves().to_string()).unwrap(),
        ));
    }
    return amounts;
}

pub fn get_amounts_in(amountOut: U512, paths: &Vec<Arc<CryptoPair>>) -> [U512; 10] {
    println!("get_amounts_in -");
    let mut amounts: [U512; 10] = [U512::zero(); 10];

    amounts[paths.len() - 1] = amountOut;
    for i in (0..paths.len()).rev() {
        println!(
            "get_amounts_in - pair - left: {}  right: {} i: {}",
            paths[i].left_id(),
            paths[i].right_id(),
            i
        );
        amounts[i - 1] = get_amount_in(
            amounts[i],
            U512::from_dec_str(&*paths[i].right_reserves().to_string()).unwrap(),
            U512::from_dec_str(&*paths[i].left_reserves().to_string()).unwrap(),
        );
    }
    return amounts;
}

pub fn full_mul(x: U512, y: U512, k: U512) -> (U512, U512) {
    let MAX_INT: U512 = U512::from_dec_str(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
    )
    .unwrap();

    let mm: U512 = x.mul(y) % MAX_INT;

    let l = x.mul(y);
    println!("mm: {}  l: {}", mm.to_string(), l.to_string());
    let mut h = mm.sub(l);
    if mm < l {
        h -= U512::from(1);
    }
    println!("mm: {}, l: {}, h: {}", mm, l, h);
    (l, h)
}

#[test]
pub fn test_full_mul() {
    let MAX_INT: U512 = U512::from_dec_str(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
    )
    .unwrap();

    let a = U512::from(22);
    let b = U512::from(13);
    let c = MAX_INT;

    let res = full_mul(a, b, c);

    println!("full_mul: {}, {}", res.0, res.1);
}

pub fn full_div(mut l: U512, mut h: U512, mut k: U512) -> U512 {
    let MAX_INT: U512 = U512::from_dec_str(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
    )
    .unwrap();

    let mut pow2 = k;
    println!("pow2: {}", pow2);
    let one = U512::from(1);
    k /= pow2;
    l /= pow2;
    println!("l: {}", l);
    l += (one + h + (MAX_INT - pow2).div(pow2));
    println!("---: {}", (MAX_INT - pow2).div(pow2) + one);
    println!("l2: {}", l);
    let two = U512::from(2);
    let mut r = U512::from(1);
    r *= two - k * r;
    r *= two - k * r;
    r *= two - k * r;
    r *= two - k * r;
    r *= two - k * r;
    r *= two - k * r;
    r *= two - k * r;
    r *= two - k * r;
    println!("l {}, r {}", l, r);
    return l * U512::from(r);
}

#[test]
pub fn test_full_div() {
    let r = full_div(U512::from(4), U512::from(3), U512::from(2));
}

pub fn mul_div(mut x: U512, mut y: U512, mut d: U512) -> U512 {
    println!("before: mul_div: x: {}  y: {} d: {}", x, y, d);
    let (mut l, mut h) = full_mul(x, y, d);
    println!("mul_div: x: {}  y: {} d: {}", x, y, d);
    if d == U512::zero() {
        println!("Found zero.");
        return U512::zero();
    }
    let mm: U512 = x.mul(y) % d;
    if mm > l {
        h -= U512::from(1);
    }
    l -= mm;
    println!("h: {}, mm: {}", h, mm);
    if h == U512::from(0) {
        if (l == U512::from(0) || d == U512::from(0)) {
            return U512::from(0);
        }
        println!("Ouch!!");
        return l / d;
    }
    println!("full_div");
    return full_div(l, h, d);
}

#[test]
pub fn test_mul_div() {
    let a = U512::from(5);
    let b = U512::from(5);
    let c = U512::from(5);

    let res = mul_div(a, b, c);
    println!("mul_div: {}", res);
}
pub fn compute_profit_maximizing_trade_2(
    truePrice1: &BigDecimal,
    truePrice2: &BigDecimal,
    reserve_first: &BigDecimal,
    reserve_second: &BigDecimal,
) -> Option<(bool, BigDecimal)> {
    /*
     if reserve_first == U512::zero()
         || reserve_second == U512::zero()
         || truePrice1 == U512::zero()
         || truePrice2 == U512::zero()
     {
         return None;
     }

    */

    let atob: bool = reserve_first.mul(truePrice2).div(reserve_second) < *truePrice1;
    println!("compute_profit_maximizing_trade_2 - atob: {}", atob);
    let invariant = reserve_first.mul(reserve_second);
    let numerator = if atob { truePrice1 } else { truePrice2 };
    let denominator = if atob { truePrice2 } else { &truePrice1 };

    let rightSide = if atob {
        reserve_first.mul(BigDecimal::from(1000))
    } else {
        reserve_second
            .mul(BigDecimal::from(1000))
            .div(BigDecimal::from(997))
    };
    let leftSide = (invariant * (numerator / denominator))
        .mul(BigDecimal::from(1000))
        .div(BigDecimal::from(997))
        .sqrt()
        .unwrap();

    if &leftSide < &rightSide {
        return None;
    }
    let digits = (&leftSide - &rightSide).digits();
    let dec_str = &*(&leftSide - &rightSide).to_string();

    let str = dec_str.split_once(".").unwrap();
    let res = U512::from_dec_str(str.0).unwrap();

    return Some((atob, BigDecimal::from_str(str.0).unwrap()));
}

// computes the direction and magnitude of the profit-maximizing trade
pub fn compute_profit_maximizing_trade(
    truePrice1: U512,
    truePrice2: U512,
    reserve_first: U512,
    reserve_second: U512,
) -> Option<(bool, U512)> {
    let atob = mul_div(reserve_first, truePrice2, reserve_second) < truePrice1;
    println!("compute_profit_maximizing_trade - atob: {}", atob);
    let true_token_1: U512 = if atob { truePrice1 } else { truePrice2 };
    let true_token_2: U512 = if atob { truePrice2 } else { truePrice1 };

    let true_reserve_1: U512 = if atob {
        reserve_first.mul(1000)
    } else {
        reserve_second.mul(1000)
    } / 997;
    let true_reserve_2: U512 = if atob { truePrice2 } else { truePrice1 };

    let invariant = reserve_first.mul(reserve_second);
    let mul_div1 = mul_div(invariant.mul(1000), true_token_1, true_token_2.mul(997));

    let leftSide: U512 = mul_div1.integer_sqrt();

    let rightSide: U512 = U512::from(true_reserve_1);
    if leftSide < rightSide {
        println!("compute_profit_maximizing_trade - bail.");
        return None;
    };
    println!(
        "compute_profit_maximizing_trade - atob: {}  amountIn: {}",
        atob,
        leftSide.sub(rightSide)
    );
    return Some((atob, leftSide.sub(rightSide)));
}

#[test]
pub fn test_compute_profit_trade() {
    let truePrice1 = U512::from(1);
    let truePrice2 = U512::from(2);
    let reserve_first = U512::from_dec_str("54690692826912932826357").unwrap();
    let reserve_second = U512::from_dec_str("3802282977229211022").unwrap();

    let result =
        compute_profit_maximizing_trade(truePrice1, truePrice2, reserve_first, reserve_second);
    println!("Result: {}", result.unwrap().1);
}

pub fn get_amount_out_2(amount_out: f64, reserve_in: f64, reserve_out: f64) -> f64 {
    return (0.997 * amount_out * reserve_in / (reserve_out - amount_out * 0.997)).add(1.0);
}

pub fn get_amount_in_2(amount_out: f64, reserve_in: f64, reserve_out: f64) -> f64 {
    return (0.997 * amount_out * reserve_in / (reserve_out - amount_out * 0.997)).add(1.0);
}

pub fn arb_index(A0: &U256, A1: &U256, B0: &U256, B1: &U256) -> BigDecimal {
    let a0 = BigDecimal::from_str(&*A0.to_string()).unwrap();
    let a1 = BigDecimal::from_str(&*A1.to_string()).unwrap();
    let b0 = BigDecimal::from_str(&*B0.to_string()).unwrap();
    let b1 = BigDecimal::from_str(&*B1.to_string()).unwrap();

    return (a0 / a1) / (b0 / b1);
    //* (*C0 / *C1) * (*D0 / *D1) * (*E0 / *E1) * (*F0 / *F1)  )
}

/*
   // q = R0a * R1b
   // r = R1a * R0b
   // s = R0a + R1a
   // if  r > q, exit
   // r2 = r^2i32
   // x_opt =   sqrt(r2 + ((q*r-r2)/s))) - r
   // if x_opt == 0u128, bail.
   // alt_amount = R0a * x_opt / R0b + x_opt)
   // p = (q * x_opt) / (r +s * x_opt) - x_opt;

*/
pub fn estimated_profit(
    buyAReserves: U512,
    buyBReserves: U512,
    sellAReserves: U512,
    sellBReserves: U512,
) {
    let kBuy = BigDecimal::from_str(&*(buyAReserves * buyBReserves).to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let kSell = BigDecimal::from_str(&*(sellAReserves * sellBReserves).to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let gamma = 0.997;
    let one = 1.0;

    let buy_a_reserves = BigDecimal::from_str(&*buyAReserves.to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let buy_b_reserves = BigDecimal::from_str(&*buyBReserves.to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let sell_a_reserves = BigDecimal::from_str(&*sellAReserves.to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let sell_b_reserves = BigDecimal::from_str(&*sellBReserves.to_string())
        .unwrap()
        .to_f64()
        .unwrap();

    let numeratorA = (kSell.sqrt() * buy_a_reserves);
    let numeratorB = (kBuy.sqrt() * sell_b_reserves);
    let denominator = kBuy.sqrt() + kSell.sqrt() * gamma;
    // const numeratorA = (kSell ** .5) * buyTokenReserves;
    //const numeratorB = (gamma ** -1) * ((kBuy ** .5) * sellTokenReserves)
    //const denominator = (kBuy ** .5) + (kSell ** .5)

    let _deltaAlpha = numeratorA - numeratorB / denominator;

    let betaDenominator = buy_b_reserves - _deltaAlpha;

    let _deltaBeta = ((one / gamma) * ((kBuy / betaDenominator) - buy_a_reserves));
    let betaPrimeDenominator = (sell_b_reserves + (gamma) * _deltaAlpha);
    let _deltaBetaPrime = (sell_a_reserves - (kSell / betaPrimeDenominator));

    let profit = _deltaBetaPrime - _deltaBeta;

    /*
        const _deltaAlpha = (numeratorA - numeratorB) / denominator;
        const betaDenominator = buyTokenReserves - _deltaAlpha;
        const _deltaBeta = (gamma ** -1) * ((kBuy / betaDenominator) - buyWETHReserves)
        const betaPrimeDenominator = sellTokenReserves + (gamma * _deltaAlpha)
        const _deltaBetaPrime = sellWETHReserves - (kSell / betaPrimeDenominator);

        const profit = _deltaBetaPrime - _deltaBeta;
    */

    dbg!(
        buy_a_reserves,
        buy_b_reserves,
        sell_a_reserves,
        sell_b_reserves,
        kBuy,
        kSell,
        numeratorA,
        numeratorB
    );
    dbg!(
        profit,
        denominator,
        _deltaAlpha,
        betaDenominator,
        _deltaBeta,
        betaPrimeDenominator,
        _deltaBetaPrime
    );
}

#[test]
pub fn test_method_b() {}

pub fn optimize_a_prime(
    a1: f64,
    b1: f64,
    a2: f64,
    b2: f64,
    a3: f64,
    b3: f64,
) -> Option<(f64, f64, f64, f64, f64)> {
    let Ea = (1.000 * a1 * a2) / (1.000 * a2 + 0.997 * b1);
    let Eb = (0.997 * b1 * b2) / (1.000 * a2 + 0.997 * b1);

    /*
        if Ea > Eb {
            return None;
        }
    */
    let optimal: f64 = (((Ea * Eb * 0.997 * 1.000) - Ea * 1.000) / 0.997);
    let delta_a = optimal.sqrt();
    let delta_b = (b1 * 0.997 * delta_a) / (a1 + 0.997 * delta_a);
    let delta_c = (b2 * 0.997 * delta_b) / (a2 + 0.997 * delta_b);
    let delta_a_prime = (b3 * 0.997 * delta_c) / (a3 + 0.997 * delta_c);
    let profit = delta_a_prime - delta_a;
    /*
        if profit <= 0.0 {
            return None;
        }
    */

    return Some((delta_a, delta_b, delta_c, delta_a_prime, profit));
}

pub fn method_c(token_a_left: U512, token_a_right: U512, token_b_left: U512, token_b_right: U512) {
    // Uniswap return U112
    let q = BigDecimal::from_str(&*(token_a_left * token_b_right).to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let r = BigDecimal::from_str(&*(token_b_left * token_a_right).to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let s = BigDecimal::from_str(&*(token_a_left + token_b_left).to_string())
        .unwrap()
        .to_f64()
        .unwrap();

    let r2 = r.pow(2i32);

    let x_opt = (r2 + ((q * r - r2) / s)).sqrt() - r;

    let token_a_left_dec = BigDecimal::from_str(&*(token_a_left).to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let token_a_right_dec = BigDecimal::from_str(&*(token_a_right).to_string())
        .unwrap()
        .to_f64()
        .unwrap();
    let alt_amount = token_a_left_dec * x_opt / token_a_right_dec + x_opt;
    let p = (q * x_opt) / (r + s * x_opt) - x_opt;

    dbg!(q, r, s, r2, x_opt, alt_amount, p);

    //return Some((x_opt, alt_amount, p));
}

#[test]
pub fn test() {
    println!("Test");
    print!("test");
    dbg!("test");

    estimated_profit(
        U512::from_dec_str("1842208280352161713545").unwrap(),
        U512::from_dec_str("520139454114812358292975").unwrap(),
        U512::from_dec_str("173104761817512388434611").unwrap(),
        U512::from_dec_str("398580838174598015836773").unwrap(),
    );
    let r = get_amount_out(
        "1871429067395935300".parse().unwrap(),
        "1842208280352161713545".parse().unwrap(),
        "1930853909833275722".parse().unwrap(),
    );
    println!("amount_out: {} ", r);
    /*
    method_b(
        U512::from_dec_str("1842208280352161713545")
            .unwrap()
            .as_u128() as f64,
        U512::from_dec_str("1930853909833275722").unwrap().as_u128() as f64,
        U512::from_dec_str("51546145877298045978409")
            .unwrap()
            .as_u128() as f64,
        U512::from_dec_str("398580838174598015836773")
            .unwrap()
            .as_u128() as f64,
        U512::from_dec_str("173104761817512388434611")
            .unwrap()
            .as_u128() as f64,
        U512::from_dec_str("398580838174598015836773")
            .unwrap()
            .as_u128() as f64,
    );

     */
    let true1 = U512::from_dec_str("1842208280352161713545").unwrap();
    let true2 = U512::from_dec_str("1930853909833275722").unwrap();
    let d = compute_profit_maximizing_trade(true1 / true2, true2 / true1, true1, true2).is_none();
    if d {
        println!("None");
    } else {
        println!("compute_max: {}", d);
    }
}

pub fn cfmm_route(A0: U256, A1: U256, B0: U256, B1: U256, C0: U256, C1: U256) {
    let R0 = &*A0.to_string();
    let R1 = &*A1.to_string();
    let R2 = &*B0.to_string();
    let R3 = &*B1.to_string();
    let R4 = &*C0.to_string();
    let R5 = &*C1.to_string();

    let cfmm = vec![
        String::from(R0.to_owned() + "," + R1 + "," + "0.997" + ",1" + "," + "2"),
        String::from(R2.to_owned() + "," + R3 + "," + "0.997" + ",2" + "," + "1"),
        String::from(R4.to_owned() + "," + R5 + "," + "0.997" + ",2" + "," + "1"),
    ];
    dbg!("cfmm route: {#:?}", route_cfmms(&cfmm));
}

pub fn reserves_to_amount(reserve0: u128, decimal0: i32, reserve1: u128, decimal1: i32) -> f64 {
    return f64::powi(10.0, (decimal0 - decimal1).abs()) * reserve1 as f64 / reserve0 as f64;
}

pub fn amount_to_reserves(reserve0: u128, decimal0: i32, reserve1: u128, decimal1: i32) -> f64 {
    return f64::powi(10.0, (decimal0 - decimal1).abs()) * reserve1 as f64 / reserve0 as f64;
}

#[test]
fn test1() {
    // Uniswap v2 pair: 0x60B2cC2c6ECD3DD89b4fD76818EF83186e2F2931
    // Sushi V2 factory: 0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac
    // Sushi V2 Pair: 0xf55C33D94150d93c2cfb833bcCA30bE388b14964

    let last_uni_0 = U256::from_dec_str("52381923901833867643199").unwrap();
    let last_uni_1 = U256::from_dec_str("5213960251881795628").unwrap();

    let last_sushi_0 = U256::from_dec_str("1603960182032743190540175").unwrap();
    let last_sushi_1 = U256::from_dec_str("162076330230527000492").unwrap();

    let before_arb_uni_0 = U256::from_dec_str("51724537986393344619269").unwrap();
    let before_arb_uni_0 =
        before_arb_uni_0.add(U256::from_dec_str("35691219869310873117201").unwrap());

    let before_arb_uni_1 = U256::from_dec_str("5267237245650456340").unwrap();
    let before_arb_uni_1 = before_arb_uni_1.sub(U256::from_dec_str("2146752548113640589").unwrap());

    let before_arb_sushi_0 = U256::from_dec_str("1622404745747121387746096").unwrap();
    let before_arb_sushi_1 = U256::from_dec_str("160084144837634851024").unwrap();

    let x0_uni_0 = U256::from_dec_str("51724537986393344619269").unwrap();
    let x0_uni_1 = U256::from_dec_str("5267237245650456340").unwrap();

    let x0_sushi_0 = U256::from_dec_str("1622404745747121387746096").unwrap();
    let x0_sushi_1 = U256::from_dec_str("160084144837634851024").unwrap();

    let x1_uni_0 = U256::from_dec_str("51724537986393344619269").unwrap();
    let x1_uni_1 = U256::from_dec_str("5267237245650456340").unwrap();

    let x1_sushi_0 = U256::from_dec_str("1622404745747121387746096").unwrap();
    let x1_sushi_1 = U256::from_dec_str("160084144837634851024").unwrap();

    let x2_uni_0 = U256::from_dec_str("51724537986393344619269").unwrap();
    let x2_uni_1 = U256::from_dec_str("5267237245650456340").unwrap();

    let x2_sushi_0 = U256::from_dec_str("1622404745747121387746096").unwrap();
    let x2_sushi_1 = U256::from_dec_str("160084144837634851024").unwrap();

    let x3_uni_0 = U256::from_dec_str("53875834171017993205484").unwrap();
    let x3_uni_1 = U256::from_dec_str("5068960251881795628").unwrap();

    let x3_sushi_0 = U256::from_dec_str("1626987980961613069918058").unwrap();
    let x3_sushi_1 = U256::from_dec_str("159651487086566179702").unwrap();
    /*
    method_a(
        before_arb_uni_0.clone(),
        before_arb_uni_1.clone(),
        before_arb_sushi_0.clone(),
        before_arb_sushi_1.clone(),
    );
    */
 /*
 cfmm_route(
     before_arb_uni_0.clone(),
     before_arb_uni_1.clone(),
     before_arb_sushi_0.clone(),
     before_arb_sushi_1.clone(),
 );*/
    println!(
        "get_amount_out: 1.94 WETH for {} ALPHA",
        get_amount_out_2(
            34107.0,
            before_arb_sushi_0.to_string().parse::<f64>().unwrap(),
            before_arb_sushi_1.to_string().parse::<f64>().unwrap()
        )
        .to_string()
    );

    println!(
        "get_amount_in: {} ALPHA for 3.23 WETH",
        get_amount_in_2(
            1.94,
            before_arb_sushi_0.to_string().parse::<f64>().unwrap(),
            before_arb_sushi_1.to_string().parse::<f64>().unwrap()
        )
        .to_string()
    );
}

/*
    Swap 1.948475554344979877 Ether For 33,539.923684686224530986 ALPHA On Uniswap V2
    Swap 33,539.923684686224530986 ALPHA For 3.232657751068671322 Ether On Sushiswap
              Last
              *****Uniswap v2***
              _reserve0   uint112 :  52381923901833867643199
             _reserve1   uint112 :  5213960251881795628
             _blockTimestampLast   uint32 :  1651964285

             **** SushiSwap v2***
             _reserve0   uint112 :  1603960182032743190540175
             _reserve1   uint112 :  162076330230527000492
             _blockTimestampLast   uint32 :  1651992466

           block 14719060
           **** uniswap v2***
           {
              "_reserve0": "51724537986393344619269",
              "_reserve1": "5267237245650456340",
              "_blockTimestampLast": 1651691612
            }

            *** Sushiswap V2 ***
            {
              "_reserve0": "1622404745747121387746096",
              "_reserve1": "160084144837634851024",
              "_blockTimestampLast": 1651772035
            }

             block 14719061
             ****Uniswap v2***
             "_reserve0": "51724537986393344619269",
             "_reserve1": "5267237245650456340",
             "_blockTimestampLast": 1651691612

           *** Sushiswap V2 ***
           {
             "_reserve0": "1622404745747121387746096",
             "_reserve1": "160084144837634851024",
             "_blockTimestampLast": 1651772035
           }

             block 14719062
           *** Uniswap v2****
           {
           "_reserve0":"51724537986393344619269"
           "_reserve1":"5267237245650456340"
           "_blockTimestampLast":1651691612
           }
           *** Sushiswap V2 ***
           {
             "_reserve0": "1622404745747121387746096",
             "_reserve1": "160084144837634851024",
             "_blockTimestampLast": 1651772035
           }

             block 14719063
             *** Uniswap v2****
           {
             "_reserve0": "53875834171017993205484",
             "_reserve1": "5068960251881795628",
             "_blockTimestampLast": 1651779277
           }
             *** Sushiswap V2 ***
           {
             "_reserve0": "1626987980961613069918058",
             "_reserve1": "159651487086566179702",
             "_blockTimestampLast": 1651779277
           }

}

// Method D


    let mut gamma_delta_a = gamma_a * delta_a;


        use lp_modeler::constraint;
        use lp_modeler::dsl::*;
        use lp_modeler::format::lp_format::LpFileFormat;
        use lp_modeler::solvers::{GurobiSolver, Solution, SolverTrait};

        let ref delta_a = LpInteger::new("delta_a").lower_bound(0.0);
        let ref delta_b = LpInteger::new("delta_b").lower_bound(0.0);
        let ref delta_c = LpInteger::new("delta_c").lower_bound(0.0);

        let ref deltaAa = LpInteger::new("deltaAa").lower_bound(0.0);

        let mut r0 = 100.0; // r0
        let mut r1 = 90.0; // r1

        let mut R1a = 90.0;
        let mut r2 = 100.0; // r2

        let mut R2a = 100.0;
        let mut r3 = 80.00;

        let mut problem = LpProblem::new("Optimal Amounts", LpObjective::Maximize);
        problem += (deltaAa - delta_a);
        problem += (-0.997 * delta_a * delta_b + 89.73 * delta_a - 100 * delta_b).equal(0);
        problem += ((-0.997 * delta_b * delta_c) + (99.7 * delta_b - 90 * delta_c)).equal(0);
        problem += ((-0.997 * delta_c * deltaAa) - (100 * deltaAa) + 79.76 * delta_c).equal(0);
        problem += (delta_a).ge(0);
        problem += (delta_b).ge(0);
        problem += (delta_c).ge(0);
        problem += (deltaAa).ge(0);

        let solver = lp_modeler::solvers::GurobiSolver::new();

        match solver.run(&problem) {
            Ok(solution) => {
                println!("Status {:?}", solution.status);
                for (name, value) in solution.results.iter() {
                    println!("value of {} = {}", name, value);
                }
            }
            Err(msg) => println!("{}", msg),
        }

        let output1 = "\\ Optimal Amounts

    Maximize
      obj: deltaAa - delta_a

    Subject To
      c1: 1096.7 delta_a - delta_b = 1099000
      c2: R1a + 897.3 delta_b - delta_c - 900 R1a = -0
      c3: R2a + 797.6 delta_c - deltaAa - R2a R1a = -0
      c4: delta_a >= -0
      c5: delta_b >= -0
      c6: delta_c >= -0
      c7: deltaAa >= -0
      c8: R1a >= -0
      c9: R2a >= -0

    "
        .to_string();
        let output2 = problem.to_lp_file_format();
        dbg!(" - {#:?}", output2.clone());
        let output2 = output2.split("Generals").collect::<Vec<&str>>();
        let output2 = output2[0];
        assert_eq!(output1, output2);
}

 */
