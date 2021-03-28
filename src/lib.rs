use wasm_bindgen::prelude::*;

/// Computes the principal for a loan with fixed-rate interest and fixed payments
/// to be paid off in a certain number of payments.
///
/// # Arguments
/// * interest_rate - The interest rate of the loan per payment period.
/// * payment - The payment made each period.
/// * num_periods - The number of payment periods.
#[wasm_bindgen]
pub fn solve_for_principal(interest_rate: f64, payment: f64, num_periods: f64) -> f64 {
    const EPSILON: f64 = 1e-5;

    // Initialize upper and lower bound to solution.
    let mut lo = payment / (1.0 + interest_rate);
    let mut hi = (payment * num_periods).min(0.8 * payment / interest_rate);

    let mut n = -1.0;
    let mut principal = -1.0;

    while (n - num_periods).abs() > EPSILON {
        // Update guess to midpoint between upper and lower bound
        principal = (lo + hi) / 2.0;
        n = calc_number_of_payments(principal, interest_rate, payment);
        // Initialize upper or lower bound to solution
        if n < num_periods {
            lo = principal
        } else {
            hi = principal
        }
    }
    return principal;
}

/// Computes the number of payments to pay down the principal.
/// Returns non-integer values to assist with goal seeking.
///
/// # Arguments
///
/// * `principal` - The principal of the loan.
/// * `interest_rate` - The interest rate of the loan per payment period.
/// * `payment` - The payment made each period.
#[wasm_bindgen]
pub fn calc_number_of_payments(principal: f64, interest_rate: f64, payment: f64) -> f64 {
    if payment <= principal * interest_rate {
        panic!("Payment must exceed first interest charge.");
    }
    let mut number_of_payments = 0.0;
    let mut remaining_principal = principal;
    while remaining_principal > 0.0 {
        remaining_principal += remaining_principal * interest_rate - payment;
        number_of_payments += 1.0;
    }
    number_of_payments += remaining_principal / payment;
    return number_of_payments;
}
