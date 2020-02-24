extern crate chrono;
use chrono::{Utc, TimeZone};
use crate::equations::{df};
use crate::dates::{calc_payment_dates, days_accrued, year_fract_to_date};
use crate::rates::{YieldCurve, YieldCurveFactory, HasQueryRate};

pub mod analytics;

trait HasPrice {
    fn price(&self) -> f64;
}

trait HasYld {
    fn yld(&self) -> f64;
}

trait HasPV {
    fn pv(&self) -> f64;
}

trait HasAI {
    fn ai(&self) -> f64;
}

trait HasYieldCurve {
    fn yc(&self) -> YieldCurve;
}

struct GenericBond {
    coupon: f64,
    maturity: f64,
    p: f64,
    bdc : (f64, f64)    
}

impl HasPrice for GenericBond {
    fn price(&self) -> f64 {
        100.
    }
}

impl HasAI for GenericBond {
    fn ai(&self) -> f64 {

        let val_dt = chrono::Utc.ymd(2021, 2, 1);
        let last_cpn_dt = chrono::Utc.ymd(2021, 2, 1);
        let mat_dt = chrono::Utc.ymd(2021 + self.maturity as i32 - 1, 2, 1);
        
        let pay_dates = calc_payment_dates(val_dt, Some(last_cpn_dt), mat_dt, self.p as u32) ; //HACK
        let days = days_accrued(val_dt, &pay_dates, self.bdc);
        
        (days as f64/self.bdc.1) * self.coupon
    }
}

//Should probably have this and bond (instrument) as comps of a pricer
// This looks quite OO
impl HasYieldCurve for GenericBond{
    fn yc(&self) -> YieldCurve {
        // YieldCurveFactory::create_default_curve()
        YieldCurveFactory::create_flat_curve(3.)
    } 
}

impl HasPV for GenericBond {
    fn pv(&self) -> f64 {
        let val_dt = Utc::now().date();
        let mat_dt = year_fract_to_date(val_dt, self.maturity); //Hacky maturity date for annual
        let freq = 1;

        //Assuming start of bond life, no payment yet
        let pay_dates = calc_payment_dates(val_dt, None, mat_dt, freq);

        println!("Matdate {}, Pay dates: {:?}", mat_dt, pay_dates);

        let mut i = 0;
        let cpn_pvs : f64 = pay_dates.iter().map(|dt| {   
                i = i + 1;
                let year_fract = i as f64;
                let rate = self.yc().query_rate(*dt) / 100.; //correct scale
                let dff = df(rate, year_fract);
                let cf = dff * self.coupon / self.p;
                println!("Year {}, DF {} CF {}", i, dff, cf);                
                cf

            }).sum();
            let final_rate = self.yc().query_rate(mat_dt) / 100.;
            let principle_pv = 100. * df(final_rate, self.maturity);
            println!("princple PV is {}", principle_pv);
            cpn_pvs + principle_pv
        }
    } 

impl HasYld for GenericBond {
    fn yld(&self) -> f64 {
        let cf_fract = self.coupon + ((100. - self.price())/self.maturity);
        (cf_fract / self.price())* 100.
    }
}

impl GenericBond {
    fn new(coupon: f64, maturity: f64, p: f64) -> GenericBond {
        GenericBond {coupon: coupon,
            maturity: maturity,
            p: p,
            bdc : (30., 360.),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonds_price_simple() {
        let bond = GenericBond::new(10., 5., 1.); 
        let price = bond.price();
        println!("Price for bond is {}", price);
        // assert!()
    }

    #[test]
    fn bonds_yield_simple() {
        let bond = GenericBond::new(10., 5., 1.); 
        let yld = bond.yld();
        println!("Yld for bond is {}", yld);
        // assert!()
    }

    #[test]
    fn bonds_pv_simple() {
        let bond = GenericBond::new(3.0, 5., 1.); 
        let pv = bond.pv();
        println!("PV for annual 3%, 5yr bond is {}", pv);
        
        let bond = GenericBond::new(3.0, 5., 2.); 
        let pv = bond.pv();
        println!("PV for semi-annual 3%, 5yr bond is {}", pv);
        // assert!()
    }


    #[test]
    fn bonds_ai() {
        let bond = GenericBond::new(3., 5., 1.); 
        let ai = bond.ai();
        println!("Accrued Interest for bond is {}", ai);
        // assert!()
    }

}