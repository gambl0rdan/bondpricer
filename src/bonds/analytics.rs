extern crate chrono;
use chrono::{Date, Utc, TimeZone};
use crate::equations::{df};
use crate::dates::{calc_payment_dates, year_fract_to_date};
use crate::rates::{YieldCurveFactory, YieldCurve, HasQueryRate};


fn yld_calced(val_dt : Date<Utc>, mat : f64, freq : f64, coupon : f64, price : f64) -> f64 {
    let mat_dt = year_fract_to_date(val_dt, mat); //Hacky maturity date for annual
    //Assuming start of bond life, no payment yet
    let pay_dates = calc_payment_dates(val_dt, None, mat_dt, freq as u32);

    println!("Matdate {}, Pay dates: {:?}", mat_dt, pay_dates);

    let mut var_rate = coupon;
    let mut srch_iter = 0; 
    loop {
        let mut i = 0;
        let cpn_pvs : f64 = pay_dates.iter().map(|dt| {   
                i = i + 1;
                let year_fract = i as f64;
                let rate = var_rate / (100. * freq); //correct scale
                let dff = df(rate, year_fract);
                let cf = dff * coupon / freq;
                // println!("Year={}, r={}, DF={} CF={}", i, rate, dff, cf);                
                cf

            }).sum();
            let final_rate = var_rate / (100. * freq);
            let principle_pv = 100. * df(final_rate, mat * freq);
            let cur_pv = cpn_pvs + principle_pv;
            let pv_diff = cur_pv - price;
            let pv_diff_px = (pv_diff / (cur_pv + price)/2.) * 100.;
            
            srch_iter = srch_iter + 1;
            println!("({}) Target={}, cur rate={}, PV={}, diff={}, % diff={}", srch_iter, price,
                 var_rate, cur_pv, pv_diff, pv_diff_px);
            
            //TODO: This wont work whenhave coupon < rate. Look at proper linear solver
            if pv_diff.abs() < 0.000001 {
                return var_rate;
            } else{
                var_rate = var_rate * (1. + pv_diff/100.);
            }
        }
        return 0.       
    }



fn pv_calced(val_dt : Date<Utc>, mat : f64, freq : f64, coupon : f64, yc : &YieldCurve) -> f64 {
    let mat_dt = year_fract_to_date(val_dt, mat); //Hacky maturity date for annual
    //Assuming start of bond life, no payment yet
    let pay_dates = calc_payment_dates(val_dt, None, mat_dt, freq as u32);

    println!("Matdate {}, Pay dates: {:?}", mat_dt, pay_dates);

    let mut i = 0;
    let cpn_pvs : f64 = pay_dates.iter().map(|dt| {   
            i = i + 1;
            let year_fract = i as f64;
            let rate = yc.queryRate(*dt) / (100. * freq); //correct scale
            let dff = df(rate, year_fract);
            let cf = dff * coupon / freq;
            println!("Year={}, r={}, DF={} CF={}", i, rate, dff, cf);                
            cf

        }).sum();
        let final_rate = yc.queryRate(mat_dt) / (100. * freq);
        let principle_pv = 100. * df(final_rate, mat * freq);
        println!("Final rate is {}, princple PV is {}", final_rate, principle_pv);
        cpn_pvs + principle_pv
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonds_analytics_comparisons() {
        let val_dt = Utc::now().date();
        let mat = 5.;
        let freq = 2.;
        let coupon = 3.;
        let yc = YieldCurveFactory::createFlatCurve(3.);
    
        println!("PV for bond is {}", pv_calced(val_dt, mat, freq, coupon, &yc));
        
        let coupon = 6.;
        
        println!("PV for bond is {}", pv_calced(val_dt, mat, freq, coupon, &yc));

        let mat = 6.2;
        let freq = 2.;
        let coupon = 7.25;
        let yc = YieldCurveFactory::createFlatCurve(5.94);

        println!("PV for bond is {}", pv_calced(val_dt, mat, freq, coupon, &yc));
    }

    #[test]
    fn bonds_analytics_ytm() {
        let val_dt = Utc::now().date();
        let mat = 5.;
        let freq = 2.;
        let coupon = 3.;
        let price = 100.;
    
        println!("Yld for bond is {}", yld_calced(val_dt, mat, freq, coupon, price));
        
        let coupon = 6.;
        let price = 113.83327682778175;
    
        println!("Yld for bond is {}", yld_calced(val_dt, mat, freq, coupon, price));
            
        let coupon = 2.;
        let price = 113.83327682778175;
    
        //This will run forever
        //println!("Yld for bond is {}", yld_calced(val_dt, mat, freq, coupon, price));
    } 
}