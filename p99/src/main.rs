use std::any::type_name_of_val;
//use argminmax::ArgMinMax;

// This code is from internet as Unable to find workaround for Argmax using argminmax
// it throws nightly error
// this code is completely unclear to me, explain this slowly
fn argmax<T: PartialOrd>(data: &[T]) -> Option<usize> {
    data.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
}

// this I wrote - I understand this fully
pub fn apply_func(s: Vec<i32>) -> f64 {
    (s[1] as f64) * (s[0] as f64).ln() as f64
}

// I understand this as well, except I finished by reading compiler errors and helps
pub fn splits(s: &str) -> f64 {
    let m: Vec<&str>  = s.split(",").collect(); // why collect()
    let k: Vec<i32> = m.iter().map(|s| s.parse().unwrap()).collect() ; // why collect()
    apply_func(k)
}

// Understood it, reading compilers error and understood
fn main() {
    let input = std::fs::read_to_string("./src/0099_base_exp.txt").expect("file not found");
    // file system : fs , read_to_string carries everything like read in python
    // convert this to readline first then break the string with comma separated then convert every number in int
    // then take log
    println!("{:?}", type_name_of_val(&input));
    
    let parts: Vec<&str> = input.split("\n").collect(); // why collect()? it doesn't work without collect

    //let result: f64 = splits(&parts[0]);
    // [0] is very different so how do I pass .get() here? it doesn't work - In case of single case
    //println!("{:?}", result);
    let result:Vec<f64> = parts.iter().map(|c| splits(c)).collect() ;
    let value: usize = argmax(&result).expect("Not found") ;
    println!("{:?}", value);
    
    
}
