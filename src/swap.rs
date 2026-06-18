const MAX_BPS: i128 = 10000;

pub fn calculate_swap_out(
    reserve_in: i128,
    reserve_out: i128,
    amount_in: i128,
    fee_bps: u32,
) -> i128 {
    assert!(reserve_in > 0 && reserve_out > 0, "insufficient liquidity");
    assert!(amount_in > 0, "amount in must be positive");
    assert!((fee_bps as i128) < MAX_BPS, "fee exceeds maximum");

    let fee_amount = amount_in
        .checked_mul(fee_bps as i128)
        .expect("overflow in fee calculation")
        .checked_div(MAX_BPS)
        .expect("division by zero in fee calculation");

    let amount_in_after_fee = amount_in
        .checked_sub(fee_amount)
        .expect("fee exceeds amount in");

    let numerator = amount_in_after_fee
        .checked_mul(reserve_out)
        .expect("overflow in numerator");

    let denominator = reserve_in
        .checked_add(amount_in_after_fee)
        .expect("overflow in denominator");

    assert!(denominator > 0, "denominator must be positive");

    numerator
        .checked_div(denominator)
        .expect("division by zero in swap calculation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_out_calculation() {
        let reserve_a: i128 = 1_000_000;
        let reserve_b: i128 = 500_000;
        let amount_in: i128 = 100_000;

        let amount_out = calculate_swap_out(reserve_a, reserve_b, amount_in, 30);
        assert!(amount_out > 0, "swap out must be positive");

        let fee = amount_in * 30 / 10000;
        let net_in = amount_in - fee;
        let expected_out = net_in * reserve_b / (reserve_a + net_in);
        assert_eq!(amount_out, expected_out, "swap calculation mismatch");
    }

    #[test]
    fn test_invariant_holds_after_swap() {
        let reserve_a: i128 = 1_000_000;
        let reserve_b: i128 = 500_000;
        let k_before = reserve_a * reserve_b;

        let amount_in: i128 = 50_000;
        let amount_out = calculate_swap_out(reserve_a, reserve_b, amount_in, 30);

        let fee = amount_in * 30 / 10000;
        let net_in = amount_in - fee;

        let new_reserve_a = reserve_a + net_in;
        let new_reserve_b = reserve_b - amount_out;
        let k_after = new_reserve_a * new_reserve_b;

        assert!(k_after >= k_before, "invariant violated: k decreased");
    }

    #[test]
    fn test_zero_fee_swap() {
        let reserve_a: i128 = 1_000_000;
        let reserve_b: i128 = 500_000;
        let amount_in: i128 = 100_000;

        let amount_out = calculate_swap_out(reserve_a, reserve_b, amount_in, 0);
        assert!(amount_out > 0);
    }

    #[test]
    fn test_small_swap() {
        let reserve_a: i128 = 1_000_000_000;
        let reserve_b: i128 = 500_000_000;
        let amount_in: i128 = 1000;

        let amount_out = calculate_swap_out(reserve_a, reserve_b, amount_in, 30);
        assert!(amount_out > 0, "even smallest swap should produce output");
    }

    #[test]
    #[should_panic(expected = "insufficient liquidity")]
    fn test_zero_reserve_panics() {
        calculate_swap_out(0, 500_000, 100_000, 30);
    }

    #[test]
    #[should_panic(expected = "amount in must be positive")]
    fn test_zero_amount_panics() {
        calculate_swap_out(1_000_000, 500_000, 0, 30);
    }
}
