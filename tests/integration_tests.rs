extern crate bondpricer;
use bondpricer::equations;
mod common;

#[test]
fn fv_from_pv() {
    let pv = 100_000.;
    let rate = 0.001; // 0.1%

    // let fv = ;
    
    // println!("Result={}", &fv);
    let res = bondpricer::equations::future_value(pv, rate, 1);

    assert!(common::assertAlmostEquals(1., 1.000002));

    // assert!(common::assertAlmostEquals!(100_100., res, ulps=2));
}