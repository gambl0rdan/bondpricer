pub fn assertAlmostEquals(a : f64, b :f64) -> bool {
    return (a - b).abs() < 0.00001; 
}