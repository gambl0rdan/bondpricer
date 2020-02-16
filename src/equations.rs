
pub fn future_value(present_value : f64, rate : f64, years : i32) -> f64{
    present_value * ((1. + rate).powi(years))
}

pub fn future_value_mp(present_value : f64, rate : f64, years : i32, p : i32) -> f64{
    present_value * ((1. + (rate/p as f64)).powi(years * p))
}

pub fn present_value(future_value : f64, rate : f64, years : i32) -> f64{
    future_value / ((1. + rate).powi(years))
}

pub fn present_value_mp(future_value : f64, rate : f64, years : i32, p : i32) -> f64{
    future_value / ((1. + (rate/p as f64)).powi(years * p))
}


pub fn future_value_cap(present_value : f64, rate : f64, act : f64, basis : f64) -> f64{
    present_value * (1. + (rate * (act/basis)))
}

pub fn present_value_cap(future_value : f64, rate : f64, years : i32, p : i32) -> f64{
    future_value / ((1. + (rate/p as f64)).powi(years * p))
}

pub fn df(rate : f64, years : f64) -> f64{
    1. / ((1. + rate).powf(years))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fv_from_pv() {
        let pv = 100_000.;
        let rate = 0.01; // 1%

        assert_eq!(101_000., future_value(pv, rate, 1));
    }

    #[test]
    fn pv_from_fv() {
        let fv = 101_000.;
        let rate = 0.01; // 1%

        assert_eq!(100_000., present_value(fv, rate, 1));
    }

    #[test]
    fn simple_discount_factors() {
        let rate = 0.01; // 1%
        let res = df(rate, 1.);
        assert!((0.99 - res).abs() < 0.01);
    }
}