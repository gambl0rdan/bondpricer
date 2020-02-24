extern crate chrono;
use chrono::{Date, Utc, TimeZone};
use crate::equations::{df};
use crate::dates::{calc_payment_dates, year_fract_to_date};
use crate::rates::{YieldCurveFactory, YieldCurve, HasQueryRate};

fn ytm_and_zspread(val_dt : Date<Utc>, mat : f64, freq : f64, coupon : f64, price : f64) -> (f64, f64) {
    let maturities = vec![1., 2., 3., 4., 5., 6.]; 
    let p = 100.;
    // bond price
    let b = 144.81; // looking for 2% yld and 0% cpn
    let b = 90.47; //looking for 7% yield 5% cpn   
    let mut cashflows : [f64; 6] =  [0.; 6];
    let mut year_fracts : [f64; 6] =  [5.; 6];

    for (i, row) in  maturities.into_iter().enumerate(){
        year_fracts[i] = row;
        cashflows[i] = (p * coupon/100.) / 1.0;
    };
    cashflows[5] = cashflows[5] + 100.; 

    let val_dt = Utc::now().date();
    let mat_dt = year_fract_to_date(val_dt, 6.); //Hacky maturity date for annual
    let freq = 1;

    //Assuming start of bond life, no payment yet
    let pay_dates = calc_payment_dates(val_dt, None, mat_dt, freq);

    println!("year_fracts {:?}", year_fracts);
    println!("Pay Dates {:?}", pay_dates);
    println!("Cashflows {:?}", cashflows);

    let init_guess = coupon/100.;
    let yc = YieldCurveFactory::create_default_curve();
    // let yc = YieldCurveFactory::create_flat_curve(1.);
    let yld = calculate_yield(init_guess, &cashflows, &year_fracts, b) * 100.;
    let zspread = calculate_z_spread(init_guess, &cashflows, &pay_dates, &year_fracts, b, &yc) * 10000.;

    (yld, zspread)
}

// Solve for yield using Newton-Raphson method
fn  calculate_yield(init_guess : f64, cashflows : &[f64], year_fracts : &[f64], b : f64) -> f64 {
        
    let error : f64 = 0.000000001;
    let mut x_i : f64 = init_guess-1.0;
    let mut x_i_next : f64 = init_guess;

    println!("Initial: PV={}, initial_guess={}, x_i={}, x_i_next={}", b, init_guess, x_i, x_i_next);
    while (x_i_next - x_i).abs() > error {
        x_i = x_i_next;
        let mut nums : Vec<f64> = vec![];
        let mut denoms: Vec<f64>  = vec![];
 
        for it in cashflows.iter().zip(year_fracts.iter()) {
            //cf, year fract
            let (cf, y) = it;
            
            // let n = x * std::f64::consts::E.powf(y *-1.*x_i);
            // let d = y * x *std::f64::consts::E.powf(y *-1.*x_i);
            let n = cf * df(x_i, *y);
            let d = y * cf * df(x_i, *y);

            nums.push(n);
            denoms.push(d);
        }

        let numerator : f64 = nums.iter().sum();
        let denominator : f64 = denoms.iter().sum();
        x_i_next = x_i + (numerator - b) / denominator;
        println!("xi+1=xi+(n-B)/d === {}={}+({}-{})/{}", x_i_next, x_i, numerator, b, denominator);
    }
    x_i_next
}

// Solve for Z-spread using Newton-Raphson method
fn calculate_z_spread(init_guess : f64, cashflows : &[f64], pay_dates : &Vec<Date<Utc>>, year_fracts : &[f64], b : f64, yc : &YieldCurve) -> f64 {
        
    let error : f64 = 0.000000001;
    let mut x_i : f64 = init_guess-1.0;
    let mut x_i_next : f64 = init_guess;
    
    println!("Initial: PV={}, initial_guess={}, x_i={}, x_i_next={}", b, init_guess, x_i, x_i_next);
    while (x_i_next - x_i).abs() > error {
        x_i = x_i_next;
        let mut nums : Vec<f64> = vec![];
        let mut denoms : Vec<f64>  = vec![];
 
        let  mut pd_iter = pay_dates.iter();
        for it in cashflows.iter().zip(year_fracts.iter()) {
            //cf, year fract
            let (cf, y) = it;
            let dt = pd_iter.next();
            let rate = yc.query_rate(*dt.unwrap()) / 100.; //correct scale
            
            let n = cf * df(x_i + rate, *y);
            let d = y * cf * df(x_i + rate, *y);

            println!("cf={} y={} dt={} rate={} n(guess)={}", cf, y, *dt.unwrap(), rate, x_i);

            nums.push(n);
            denoms.push(d);
        }

        let numerator : f64 = nums.iter().sum();
        let denominator : f64 = denoms.iter().sum();
        x_i_next = x_i + (numerator - b) / denominator;
        // println!("xi+1=xi+(n-B)/d === {}={}+({}-{})/{}", x_i_next, x_i, numerator, b, denominator);
    }
    x_i_next
}


fn calculate_yield_naive_solver(val_dt : Date<Utc>, mat : f64, freq : f64, coupon : f64, price : f64) -> f64 {
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
            let rate = yc.query_rate(*dt) / (100. * freq); //correct scale
            let dff = df(rate, year_fract);
            let cf = dff * coupon / freq;
            println!("Year={}, r={}, DF={} CF={}", i, rate, dff, cf);                
            cf

        }).sum();
        let final_rate = yc.query_rate(mat_dt) / (100. * freq);
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
    fn bonds_analytics_ytm_zspread() {
        let val_dt = Utc::now().date();
        let mat = 6.;
        let freq = 2.;
        let coupon = 2.;
        let price = 100.;
    
        let (yld, zspread) = ytm_and_zspread(val_dt, mat, freq, coupon, price);
        println!("Yld={}, Z-Spread={}", yld, zspread);
    }

    #[test]
    fn bonds_analytics_ytm() {
        let val_dt = Utc::now().date();
        let mat = 5.;
        let freq = 2.;
        let coupon = 3.;
        let price = 100.;
    
        println!("Yld for bond is {}", calculate_yield_naive_solver(val_dt, mat, freq, coupon, price));
        
        let coupon = 6.;
        let price = 113.83327682778175;
    
        println!("Yld for bond is {}", calculate_yield_naive_solver(val_dt, mat, freq, coupon, price));
            
        // let coupon = 2.;
        // let price = 113.83327682778175;
        //This will run forever
        //println!("Yld for bond is {}", yld_calced(val_dt, mat, freq, coupon, price));
    } 
}