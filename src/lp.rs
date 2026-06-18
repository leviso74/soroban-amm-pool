use crate::LiquidityPool;

pub fn calculate_initial_lp(amount_a: i128, amount_b: i128) -> i128 {
    assert!(amount_a > 0 && amount_b > 0, "amounts must be positive for initial liquidity");

    let geometric_mean = {
        let product = amount_a
            .checked_mul(amount_b)
            .expect("overflow in product");

        sqrt(product)
    };

    assert!(geometric_mean > 0, "geometric mean must be positive");
    geometric_mean
}

pub fn calculate_lp_mint(pool: &LiquidityPool, amount_a: i128, amount_b: i128) -> i128 {
    assert!(pool.lp_token_supply > 0, "pool has no liquidity");
    assert!(amount_a > 0 && amount_b > 0, "amounts must be positive");
    assert!(pool.reserve_a > 0 && pool.reserve_b > 0, "pool reserves must be positive");

    let share_a = amount_a
        .checked_mul(pool.lp_token_supply)
        .expect("overflow")
        .checked_div(pool.reserve_a)
        .expect("division by zero");

    let share_b = amount_b
        .checked_mul(pool.lp_token_supply)
        .expect("overflow")
        .checked_div(pool.reserve_b)
        .expect("division by zero");

    if share_a < share_b { share_a } else { share_b }
}

pub fn calculate_withdraw(pool: &LiquidityPool, lp_amount: i128) -> (i128, i128) {
    assert!(lp_amount > 0, "lp amount must be positive");
    assert!(pool.lp_token_supply > 0, "pool has no liquidity");
    assert!(lp_amount <= pool.lp_token_supply, "lp amount exceeds supply");

    let amount_a = lp_amount
        .checked_mul(pool.reserve_a)
        .expect("overflow")
        .checked_div(pool.lp_token_supply)
        .expect("division by zero");

    let amount_b = lp_amount
        .checked_mul(pool.reserve_b)
        .expect("overflow")
        .checked_div(pool.lp_token_supply)
        .expect("division by zero");

    assert!(amount_a > 0 && amount_b > 0, "withdraw amounts must be positive");
    (amount_a, amount_b)
}

fn sqrt(n: i128) -> i128 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (y + n / y) / 2;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LiquidityPool;
    use soroban_sdk::testutils::Address as AddressTestUtilsTrait;

    fn make_pool(reserve_a: i128, reserve_b: i128, lp_supply: i128) -> LiquidityPool {
        let env = soroban_sdk::Env::default();
        LiquidityPool {
            token_a: soroban_sdk::Address::generate(&env),
            token_b: soroban_sdk::Address::generate(&env),
            reserve_a,
            reserve_b,
            lp_token_supply: lp_supply,
            fee_bps: 30,
        }
    }

    #[test]
    fn test_initial_lp_calculation() {
        let lp = calculate_initial_lp(1_000_000, 500_000);
        assert!(lp > 0, "initial lp must be positive");
        let expected = sqrt(1_000_000 * 500_000);
        assert_eq!(lp, expected);
    }

    #[test]
    fn test_lp_mint_proportional() {
        let pool = make_pool(1_000_000, 500_000, 100_000);
        let lp = calculate_lp_mint(&pool, 100_000, 50_000);
        let expected_a = 100_000 * 100_000 / 1_000_000;
        let expected_b = 50_000 * 100_000 / 500_000;
        assert_eq!(lp, expected_a);
        assert_eq!(expected_a, expected_b, "proportional deposit should yield equal shares");
    }

    #[test]
    fn test_lp_mint_uses_smaller_share() {
        let pool = make_pool(1_000_000, 500_000, 100_000);
        let lp = calculate_lp_mint(&pool, 100_000, 30_000);
        let expected_b = 30_000 * 100_000 / 500_000;
        assert_eq!(lp, expected_b, "should use the smaller ratio");
    }

    #[test]
    fn test_withdraw_full_lp() {
        let pool = make_pool(1_000_000, 500_000, 100_000);
        let (a, b) = calculate_withdraw(&pool, 100_000);
        assert_eq!(a, 1_000_000, "full withdrawal should return all tokens");
        assert_eq!(b, 500_000, "full withdrawal should return all tokens");
    }

    #[test]
    fn test_withdraw_half_lp() {
        let pool = make_pool(1_000_000, 500_000, 100_000);
        let (a, b) = calculate_withdraw(&pool, 50_000);
        assert_eq!(a, 500_000, "half withdrawal should return half tokens");
        assert_eq!(b, 250_000, "half withdrawal should return half tokens");
    }

    #[test]
    fn test_fee_proportion_on_withdraw() {
        let pool = make_pool(1_000_000, 500_000, 100_000);
        let lp_initial = pool.lp_token_supply;

        let (withdraw_a, withdraw_b) = calculate_withdraw(&pool, lp_initial);
        assert_eq!(withdraw_a, pool.reserve_a);
        assert_eq!(withdraw_b, pool.reserve_b);
    }

    #[test]
    #[should_panic(expected = "amounts must be positive for initial liquidity")]
    fn test_initial_lp_zero_amount() {
        calculate_initial_lp(0, 500_000);
    }

    #[test]
    #[should_panic(expected = "lp amount exceeds supply")]
    fn test_withdraw_exceeds_supply() {
        let pool = make_pool(1_000_000, 500_000, 100_000);
        calculate_withdraw(&pool, 200_000);
    }
}
