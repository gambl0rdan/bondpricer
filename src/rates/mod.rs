extern crate chrono;
use chrono::{Date, DateTime, Duration, Utc};

pub trait HasQueryRate {
    fn queryRate(&self, date : Date<Utc>) -> f64;
}



pub struct YieldCurve {
    rates : Vec<(Date<Utc>, f64)>   
}

impl YieldCurve {
    pub fn new(inputRates : Vec<(Date<Utc>, f64)> ) -> YieldCurve {
        YieldCurve {
            rates : inputRates
        }
    }
}

pub struct YieldCurveFactory {
}
impl YieldCurveFactory {
    pub fn createFlatCurve(rate : f64) -> YieldCurve {
        let today = Utc::now().date();
        let input_rates = vec![ 
            (today.checked_add_signed(Duration::days(1)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(30 * 1)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(30 * 3)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(30 * 6)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(30 * 9)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 1)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 2)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 3)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 4)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 6)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 8)).unwrap(), rate),
            (today.checked_add_signed(Duration::days(365 * 10)).unwrap(), rate)
        ];
        YieldCurve::new(input_rates)
    }

    pub fn createDefaultCurve() -> YieldCurve {
        let today = Utc::now().date();
        let input_rates = vec![ 
            (today.checked_add_signed(Duration::days(1)).unwrap(), 0.2),
            (today.checked_add_signed(Duration::days(30 * 1)).unwrap(), 0.4),
            (today.checked_add_signed(Duration::days(30 * 3)).unwrap(), 0.6),
            (today.checked_add_signed(Duration::days(30 * 6)).unwrap(), 0.8),
            (today.checked_add_signed(Duration::days(30 * 9)).unwrap(), 0.9),
            (today.checked_add_signed(Duration::days(365 * 1)).unwrap(), 1.3),
            (today.checked_add_signed(Duration::days(365 * 2)).unwrap(), 1.9),
            (today.checked_add_signed(Duration::days(365 * 3)).unwrap(), 2.6),
            (today.checked_add_signed(Duration::days(365 * 4)).unwrap(), 3.4),
            (today.checked_add_signed(Duration::days(365 * 6)).unwrap(), 4.0),
            (today.checked_add_signed(Duration::days(365 * 8)).unwrap(), 4.8),
            (today.checked_add_signed(Duration::days(365 * 10)).unwrap(), 5.8)
        ];
        YieldCurve::new(input_rates)
    }
}

impl HasQueryRate for YieldCurve{
    // Just gets the first rate greater than input date. TODO: add proper interpolation
    fn queryRate(&self, valuation_date : Date<Utc>) -> f64 {
        let rate_iter = self.rates.iter()
        .find(|x| x.0 >= valuation_date);

        match rate_iter {
            Some(rate) => rate.1,
            None => 0.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yc_new_curve(){
        
        //let chrono::Utc.ymd(2021, 3, 1)
        let today = Utc::now().date();
        let base = YieldCurveFactory::createDefaultCurve();

        let res = base.queryRate(today);
        println!("Res 1!!! {}", res);
        assert!((res - 0.2).abs() < 0.00001); 
        
        let one_years_time = today.checked_add_signed(Duration::days(365 * 1)).unwrap();
        let res = base.queryRate(one_years_time);
        
        println!("Res 2!!! {}", res);
        assert!((res - 1.3).abs() < 0.00001);


    }
}