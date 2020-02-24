extern crate chrono;
use chrono::{Date, Duration, Utc, TimeZone};
use crate::equations::{df};
use crate::dates::{date_to_year_fract};

pub enum InterpolationType {
    LinearDiscountFactor,
    NearestGreaterThan, //Mostly for testing
    NearestLessThan //Mostly for testing
}

pub trait HasQueryRate {
    fn query_rate(&self, date : Date<Utc>) -> f64;
}

pub struct YieldCurve {
    rates : Vec<(Date<Utc>, f64)>,
    interp_type : InterpolationType 
}

impl YieldCurve {
    pub fn new(input_dates : Vec<(Date<Utc>, f64)> ) -> YieldCurve {
        YieldCurve {
            rates : input_dates,
            interp_type : InterpolationType::LinearDiscountFactor
        }
    }
}

pub struct YieldCurveFactory {
}
impl YieldCurveFactory {
    pub fn create_flat_curve(rate : f64) -> YieldCurve {
        let today = Utc::now().date();
        let input_rates = vec![ 
            (today, 0.0),
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

    pub fn create_default_curve() -> YieldCurve {
        let today = Utc::now().date();
        let input_rates = vec![ 
            (today, 0.0),
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

impl HasQueryRate for YieldCurve {

    // Just gets the first rate greater than input date. TODO: add proper interpolation
    fn query_rate(&self, valuation_date : Date<Utc>) -> f64 {
        
        //TODO: How to make this a const???
        let invalid_dt_rate : &(Date<Utc>, f64)= &(chrono::Utc.ymd(1900, 1, 1), -999999.);
        let rate_iter = self.rates.iter()
        .find(|x| x.0 >= valuation_date);

        let r_i_plus_one : &(Date<Utc>, f64) = match rate_iter {
            Some(rate) => rate,
            None => invalid_dt_rate
        };

        let rate_iter_2 = self.rates.iter().rev()
        .find(|x| x.0 <= valuation_date);
        let r_i : &(Date<Utc>, f64) = match rate_iter_2 {
            Some(rate) => rate,
            None => invalid_dt_rate
        };

        let start_date = match self.rates.first() {
            Some(dt) => dt.0,
            None => invalid_dt_rate.0
        };
        
        match self.interp_type {
            InterpolationType::LinearDiscountFactor => YieldCurve::interpolate_linear_on_df(r_i, r_i_plus_one, &start_date, &valuation_date),
            InterpolationType::NearestGreaterThan => r_i_plus_one.1,
            InterpolationType::NearestLessThan => r_i.1
        }
    }

}

impl YieldCurve{
    pub fn interpolate_linear_on_df(r_i : &(Date<Utc>, f64), r_i_plus_one : &(Date<Utc>, f64), start_date : &Date<Utc>, valuation_date : &Date<Utc>) -> f64 {
        let fr_i = date_to_year_fract(*start_date, r_i.0);
        let fr_i_plus_one = date_to_year_fract(*start_date, r_i_plus_one.0);
        let fr = date_to_year_fract(*start_date, *valuation_date);

        let p0 = ((fr - fr_i) / (fr_i_plus_one - fr_i)) * df(r_i_plus_one.1, fr_i_plus_one);
        let p1 = ((fr_i_plus_one - fr) / (fr_i_plus_one - fr_i)) * df(r_i.1, fr_i);
    
        return p0 + p1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Duration, Utc, TimeZone};

    #[test]
    fn yc_new_curve(){
        let today = Utc::now().date();
        let base = YieldCurveFactory::create_default_curve();

        let res = base.query_rate(today);
        println!("Res 1!!! {}", res);
        assert!((res - 0.2).abs() < 0.00001); 
        
        let one_years_time = today.checked_add_signed(Duration::days(365 * 1)).unwrap();
        let res = base.query_rate(one_years_time);
        
        println!("Res 2!!! {}", res);
        assert!((res - 1.3).abs() < 0.00001);
    }

    #[test]
    fn yc_linear_interpolate(){
        let start_dt = chrono::Utc.ymd(2020, 3, 1); 
        let i_dt = (chrono::Utc.ymd(2020, 6, 1), 0.5);
        let i_plus_one_dt = (chrono::Utc.ymd(2020, 12, 1), 1.);
        let val_dt = chrono::Utc.ymd(2020, 9, 1);

        let res = YieldCurve::interpolate_linear_on_df(&i_dt, &i_plus_one_dt, &start_dt, &val_dt);
        println!("Res {}", res);
        assert!((res - 0.75).abs() < 0.1);

        let i_dt = (chrono::Utc.ymd(2020, 6, 1), 5.);
        let i_plus_one_dt = (chrono::Utc.ymd(2020, 12, 1), 10.);
        
        let res = YieldCurve::interpolate_linear_on_df(&i_dt, &i_plus_one_dt, &start_dt, &val_dt);
        println!("Res {}", res);
        assert!((res - 0.4).abs() < 0.1);
    
        let i_dt = (chrono::Utc.ymd(2021, 3, 1), 5.);
        let i_plus_one_dt = (chrono::Utc.ymd(2022, 3, 1), 5.);
        let val_dt = chrono::Utc.ymd(2022, 3, 1);
        let res = YieldCurve::interpolate_linear_on_df(&i_dt, &i_plus_one_dt, &start_dt, &val_dt);
        println!("Res {}", res);
        assert!((res - 0.02777).abs() < 0.1);
    }
}