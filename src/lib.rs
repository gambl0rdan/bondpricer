pub mod bonds;
pub mod rates;
pub mod equations;
pub mod dates;

use equations::{future_value, future_value_mp, present_value, future_value_cap};

pub fn price_instrument() {
    let loan_amt = 100_000.;

    let total = future_value(loan_amt, 0.1, 1);
    println!("Total cost to repay is {}", total);

    let total = future_value_mp(loan_amt, 0.1, 1, 2);
    println!("Total cost to repay is {}", total);

    let total = future_value_cap(loan_amt, 0.1, 182.5, 365.);
    let total = -loan_amt + total + future_value_cap(loan_amt, 0.1, 182.5, 365.);
    println!("Total cost to repay is {}", total);

}