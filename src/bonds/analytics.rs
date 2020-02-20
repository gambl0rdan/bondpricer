extern crate chrono;
use chrono::{Date, Utc, TimeZone};
use crate::equations::{df};
use crate::dates::{calc_payment_dates, year_fract_to_date};
use crate::rates::{YieldCurveFactory, YieldCurve, HasQueryRate};

// use std::f64::consts;


fn ytm_2(val_dt : Date<Utc>, mat : f64, freq : f64, coupon : f64, price : f64) -> f64 {

    let maturities = vec![6., 12., 18., 24., 30., 36.];
    // 
    let maturities : Vec<f64> = maturities.into_iter().map(|x| x/12.).collect();
    let maturities = vec![1., 2., 3., 4., 5., 6.]; 
    let p = 100.;
    // bond price
    let b = 100.;
    let b = 116.8;
    let b = 144.81; // looking for 2% yld and 0% cpn
    let b = 90.47; //looking for 7% yield 5% cpn   
    let couponRate = 0.05;
    // let couponRate = 5.0;
    
    // calculate future cashflows
    //let cashflows : Vec<(f64, f64)> = maturities.into_iter().map(|i| ( i, ((p * couponRate) / 2.0)).collect();
    
    //let mut grid: [[i32; 5]; 2] = [[5; 5]; 2];
    
    // let mut cashflows : [[f64; 6]; 2] =  [[5.; 6]; 2];
    // println!("Before {:?}", cashflows);
    
    let mut cashflows : [f64; 6] =  [couponRate; 6];
    let mut yearFrac : [f64; 6] =  [5.; 6];

    // double[] cashflows = maturities.Select(i => (P * couponRate) / 2.0).ToArray();
    

    for (i, row) in  maturities.into_iter().enumerate(){
        yearFrac[i] = row;
        cashflows[i] = (p * couponRate) / 1.0;
    };
    cashflows[5] = cashflows[5] + 100.; 

    println!("yearFrac {:?}", yearFrac);
    println!("Cashflows {:?}", cashflows);
    
    let initialGuess = couponRate;
    CalculateYield(initialGuess, &cashflows, &yearFrac, b)
}

fn CalculateYield(initialGuess : f64, cashflows : &[f64], yearFrac : &[f64], B : f64) -> f64 {
        
    let error : f64 = 0.000000001;
    let mut x_i : f64 = initialGuess-1.0;
    let mut x_i_next : f64 = initialGuess;

    println!("Initial: PV={}, initial_guess={}, x_i={}, x_i_next={}", B, initialGuess, x_i, x_i_next);
    while (x_i_next - x_i).abs() > error {
        x_i = x_i_next;
        // linq's Zip is handy to perform a sum over expressions involving several arrays
        // numerator = cashflows.Zip(yearFrac, (x,y) => (x * Math.Exp(y *-1*x_i))).Sum();
        // denominator = yearFrac.Zip(cashflows, (x,y) => (x*y*Math.Exp(x*-1*x_i))).Sum();
        let mut numerator = -1.;
        let mut denominator = -1.;
        let mut nums : Vec<f64> = vec![];
        let mut denoms: Vec<f64>  = vec![];


        
        for it in cashflows.iter().zip(yearFrac.iter()) {
            //cf, year fract
            let (cf, y) = it;
            
            // let n = x * std::f64::consts::E.powf(y *-1.*x_i);
            // let d = y * x *std::f64::consts::E.powf(y *-1.*x_i);

            let n = cf * df(x_i, *y);
            let d = y * cf * df(x_i, *y);

            nums.push(n);
            denoms.push(d);
        }

        numerator = nums.iter().sum();
        denominator = denoms.iter().sum();
        x_i_next = x_i + (numerator - B) / denominator;
        println!("xi+1=xi+(n-B)/d === {} = {} + ({} - {})/{}", x_i_next, x_i, numerator, B, denominator);
    }
    x_i_next
}



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
        let yc = YieldCurveFactory::create_flat_curve(3.);
    
        println!("PV for bond is {}", pv_calced(val_dt, mat, freq, coupon, &yc));
        
        let coupon = 6.;
        
        println!("PV for bond is {}", pv_calced(val_dt, mat, freq, coupon, &yc));

        let mat = 6.2;
        let freq = 2.;
        let coupon = 7.25;
        let yc = YieldCurveFactory::create_flat_curve(5.94);

        println!("PV for bond is {}", pv_calced(val_dt, mat, freq, coupon, &yc));
    }

    #[test]
    fn bonds_analytics_ytm_2() {
        let val_dt = Utc::now().date();
        let mat = 5.;
        let freq = 2.;
        let coupon = 3.;
        let price = 100.;
    
        println!("Yld for bond is {}", ytm_2(val_dt, mat, freq, coupon, price));
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