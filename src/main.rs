extern crate bondpricer;

fn main() {
   
    // let fv = future_value(pv, rate, 1);
    
    // println!("Result={}", &fv);

    // let pv = present_value(fv, rate, 1);
    // println!("Result={}", &pv);

    bondpricer::price_instrument();

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}