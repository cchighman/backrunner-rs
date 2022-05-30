/*
# Liquidity math adapted from https://github.com/Uniswap/uniswap-v3-periphery/blob/main/contracts/libraries/LiquidityAmounts.sol
 */

pub mod uniswap_v3_liquidity {
    use bigdecimal::BigDecimal;

    use std::str::FromStr;

    fn get_liquidity_0(x: &BigDecimal, sa: &BigDecimal, sb: &BigDecimal) -> BigDecimal {
        return ((x * sa) * sb) / (sb - sa);
    }

    fn get_liquidity_1(y: &BigDecimal, sa: &BigDecimal, sb: &BigDecimal) -> BigDecimal {
        return y / (sb - sa);
    }

    fn get_liquidity(
        x: &BigDecimal,
        y: &BigDecimal,
        sp: &BigDecimal,
        sa: &BigDecimal,
        sb: &BigDecimal,
    ) -> BigDecimal {
        let mut liquidity = Default::default();
        if sp <= sa {
            liquidity = get_liquidity_0(x, sa, sb);
        } else {
            if sp < sb {
                let liquidity0 = get_liquidity_0(x, sp, sb);
                let liquidity1 = get_liquidity_1(y, sa, sp);
                liquidity = liquidity0.min(liquidity1);
            } else {
                liquidity = get_liquidity_1(y, sa, sb);
            }
        }
        return liquidity;
    }
    /*
    /* Calculate x and y given liquidity and price range */

    fn calculate_x(
        l: &BigDecimal,
        sp: &BigDecimal,
        sa: &BigDecimal,
        sb: &BigDecimal,
    ) -> BigDecimal {
        //sp.min(sb).max(sa);
        return BigDecimal::from(l * (sb - sp) / (sp * sb));
    }

    fn calculate_y(
        l: &BigDecimal,
        sp: &BigDecimal,
        sa: &BigDecimal,
        sb: &BigDecimal,
    ) -> BigDecimal {
        //sp.min(sb).max(sa);
        return BigDecimal::from(l * (sp - sa));
    }

    fn calculate_a1(l: &BigDecimal, sp: &BigDecimal, y: &BigDecimal) -> BigDecimal {
        // sqrt(a) = sqrt(P) - y / l
        let res = sp - (y / l);
        return res.square();
    }

    fn calculate_a2(
        sp: &BigDecimal,
        sb: &BigDecimal,
        x: &BigDecimal,
        y: &BigDecimal,
    ) -> BigDecimal {
        let sa = y / (sb * x) + sp - y / (sp * x);
        return sa.square();
    }

    fn calculate_b1(
        l: &BigDecimal,
        sp: &BigDecimal,
        _sa: &BigDecimal,
        x: &BigDecimal,
        _y: &BigDecimal,
    ) -> BigDecimal {
        return ((l * sp) / (l - (sp * x))).square();
    }

    fn calculate_b2(
        sp: &BigDecimal,
        sa: &BigDecimal,
        x: &BigDecimal,
        y: &BigDecimal,
    ) -> BigDecimal {
        let p = sp.square();
        return ((sp * y) / ((((sa * sp) - p) * x) + y)).square();
    }

    fn calculate_c(p: &BigDecimal, d: &BigDecimal, x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
        let res = d - BigDecimal::from(1);
        return y / (res * p * x + y);
    }


    fn calculate_d(p: &BigDecimal, c: &BigDecimal, x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
        return BigDecimal::from(1) + ((y * (BigDecimal::from(1) - c)) / ((c * p) * x));
    }

    #[test]
    fn tests() {
        test_1();
    }
    #[test]
    fn test(x: &BigDecimal, y: &BigDecimal, p: &BigDecimal, a: &BigDecimal, b: &BigDecimal) {
        let sp = p.sqrt().unwrap();
        let sa = a.sqrt().unwrap();
        let sb = b.sqrt().unwrap();
        let l = get_liquidity(x, y, &sp, &sa, &sb);
        println!("{:?} ", l);
        let mut ia = calculate_a1(&l, &sp, y);
        let mut error = BigDecimal::from(100) * (BigDecimal::from(1) - (ia / a));

        ia = calculate_a2(&sp, &sb, x, y);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (ia / a));

        let mut ib = calculate_b1(&l, &sp, &sa, x, y);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (&ib / b));

        println!("b: {} vs {}, error {}%", &b, &ib, error);

        ib = calculate_b2(&sp, &sa, x, y);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (&ib / b));
        println!("b: {} vs {}, error {}%", &b, &ib, error);

        let c = &sb / &sp;
        let d = &sa / &sp;
        let ic = calculate_c(p, &d, x, y);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (&ic / &c));
        println!("c^2: {} vs {}, error {}%", &c.square(), &ic.square(), error);

        let id = calculate_d(p, &c, x, y);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (&id.square() / &d.square()));
        println!("d^2: {} vs {}, error {}%", &d.square(), &id.square(), error);

        let ix = calculate_x(&l, &sp, &sa, &sb);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (&ix / x));
        println!("x: {} vs {}, error {}%", x, &ix, error);
        let iy = calculate_y(&l, &sp, &sa, &sb);
        error = BigDecimal::from(100) * (BigDecimal::from(1) - (&iy / y));
        println!("y: {} vs {}, error {}%", y, &iy, error);
        println!("{:?} ", "");
    }

    /*
           fn example_1() {
               println!(
                   "{:?} ",
                   "Example 1: how much of USDC I need when providing 2 ETH at this price and range?"
               );
               let p = 2000;
               let a = 1500;
               let b = 2500;
               let x = 2;
               let sp = p.pow(0.5);
               let sa = a.pow(0.5);
               let sb = b.pow(0.5);
               let L = get_liquidity_0(x, sp, sb);
               let y = calculate_y(L, sp, sa, sb);
               println!("{:?} ", "amount of USDC y={:.2f}".format(y));
               let c = (sb / sp);
               let d = (sa / sp);
               let ic = calculate_c(p, d, x, y);
               let id = calculate_d(p, c, x, y);
               let C = ic.pow(2);
               let D = id.pow(2);
               println!(
                   "{:?} ",
                   "p_a={:.2f} ({:.2f}% of P), p_b={:.2f} ({:.2f}% of P)".format(
                       (D * p),
                       (D * 100),
                       (C * p),
                       (C * 100)
                   )
               );
               println!("{:?} ", "");
           }

           fn example_2() {
               println!("{:?} ", "Example 2: I have 2 ETH and 4000 USDC, range top set to 3000 USDC. What's the bottom of the range?");
               let p = 2000;
    f           let b = 3000;
               let x = 2;
               let y = 4000;
               let sp = p.pow(0.5);
               let sb = b.pow(0.5);
               let a = calculate_a2(sp, sb, x, y);
               println!("{:?} ", "lower bound of the price p_a={:.2f}".format(a));
               println!("{:?} ", "");
           }

           fn example_3() {
               println!("{:?} ", "Example 3: Using the position created in Example 2, what are asset balances at 2500 USDC per ETH?");
               let p = 2000;
               let a = 1333.33;
               let b = 3000;
               let x = 2;
               let y = 4000;
               let mut sp = p.pow(0.5);
               let sa = a.pow(0.5);
               let sb = b.pow(0.5);
               let L = get_liquidity(x, y, sp, sa, sb);
               let P1 = 2500;
               let mut sp1 = P1.pow(0.5);
               let mut x1 = calculate_x(L, sp1, sa, sb);
               let mut y1 = calculate_y(L, sp1, sa, sb);
               println!(
                   "{:?} ",
                   "Amount of ETH x={:.2f} amount of USDC y={:.2f}".format(x1, y1)
               );
               sp = sp.iter().min().unwrap().iter().max().unwrap();
               sp1 = sp1.iter().min().unwrap().iter().max().unwrap();
               let delta_p = (sp1 - sp);
               let delta_inv_p = ((1 / sp1) - (1 / sp));
               let delta_x = (delta_inv_p * L);
               let delta_y = (delta_p * L);
               x1 = (x + delta_x);
               y1 = (y + delta_y);
               println!(
                   "{:?} ",
                   "delta_x={:.2f} delta_y={:.2f}".format(delta_x, delta_y)
               );
               println!(
                   "{:?} ",
                   "Amount of ETH x={:.2f} amount of USDC y={:.2f}".format(x1, y1)
               );
           }

           fn examples() {
               example_1();
               example_2();
               example_3();
           }

        */
        */
}
