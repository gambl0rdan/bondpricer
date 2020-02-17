extern crate chrono;
use chrono::{Date, Duration, Utc};

//We would have the enture schedules as property of a bond?
pub fn calc_payment_dates(valuation_date : Date<Utc>, last_coupon_date : Option<Date<Utc>>,  maturity_date : Date<Utc>, frequency : u32) -> Vec<Date<Utc>>{
    let mut dates : Vec<Date<Utc>> = vec![];
    let (mut period_dt, mut ignore_init_dt) : (Date<Utc>, bool) = match last_coupon_date {
        Some(dt) => (dt, false),
        None => (valuation_date, true)
    };

    let dur  = match frequency {
        1 => Duration::days(365),
        2 => Duration::days(180), //Assume 30/360
        4 => Duration::days(90), //Assume 30/360
        _ => Duration::days(365), //Need to handle better
    };
    //Assume frequency 1 | 2 | 4
    
    while period_dt <= maturity_date{
        //TODO: A functional way of this logic would be better...
        if ignore_init_dt {
            ignore_init_dt = false;
        } else{
            dates.push(period_dt);
        }

        period_dt = period_dt.checked_add_signed(dur).unwrap();
    }
    dates
}

pub fn calc_year_fraction() -> f64{
    0.
}

pub fn days_accrued(valuation_date : Date<Utc>, schedules : &Vec<Date<Utc>>, bdc : (f64, f64)) -> i64 {
    
    let broken_dt_iter = schedules.iter().rev()
        .find(|x| *x < &valuation_date);

    match broken_dt_iter {
        Some(s) => valuation_date.signed_duration_since(*s).num_days(),
        None => 0
    }
}

pub fn settle_date_from_days(valuation_date : Date<Utc>, settle_days : i64) -> Option<Date<Utc>>{
    valuation_date.checked_sub_signed(Duration::days(-settle_days))
}

pub fn year_fract_to_date(valuation_date : Date<Utc>, year_fraction : f64) -> Date<Utc> {
    let years_to_days = (year_fraction * 365.).floor() as i64;
    match valuation_date.checked_add_signed(Duration::days(years_to_days)) {
        Some(s) => s,
        None => valuation_date //TODO: Handle crash
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Duration, Utc, TimeZone};


    #[test]
    fn date_vec_flows(){
        let val_dt = chrono::Utc.ymd(2021, 3, 1);
        let last_cpn_dt = chrono::Utc.ymd(2020, 2, 1);
        let mat_dt = chrono::Utc.ymd(2024, 2, 1);
        let freq = 1;

        let res = calc_payment_dates(val_dt, Some(last_cpn_dt), mat_dt, freq);
        println!("Res is {:?}", res);

    }


    #[test]
    fn date_accrued_days_broken_dates(){
        let val_dt = chrono::Utc.ymd(2021, 3, 1);
        let last_cpn_dt = chrono::Utc.ymd(2022, 2, 1);
        let mat_dt = chrono::Utc.ymd(2024, 2, 1);
        let freq = 1;
        let bdc = (30., 360.);
        let pay_dates = calc_payment_dates(val_dt, Some(last_cpn_dt), mat_dt, freq);
        let res = days_accrued(val_dt, &pay_dates, bdc);

        assert_eq!(0, res);

        let val_dt = chrono::Utc.ymd(2022, 3, 1);
        let res = days_accrued(val_dt, &pay_dates, bdc); //TODO: Need better understanding of the ref

        assert_eq!(28, res);



    }

    #[test]
    fn date_from_1st_jan_2020(){
        let val_dt = chrono::Utc.ymd(2020, 1, 1);
        let settle_dt = chrono::Utc.ymd(2020, 1, 4);
        println!("Val {}", val_dt);
        let res = settle_date_from_days(val_dt, 3);
        assert_eq!(Some(settle_dt), res);

        let calc_settle_dt = res.unwrap();
        assert!(calc_settle_dt > val_dt);
    }
}