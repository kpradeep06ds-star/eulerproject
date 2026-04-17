use std::any::type_name_of_val;
//use argminmax::ArgMinMax;

fn argmax<T: PartialOrd>(data: &[T]) -> Option<usize> {
    data.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
}


pub fn apply_func(s: Vec<i32>) -> f64 {
    (s[1] as f64) * (s[0] as f64).ln() as f64
}

pub fn splits(s: &str) -> f64 {
    let m: Vec<&str>  = s.split(",").collect(); // why collect()
    let k: Vec<i32> = m.iter().map(|s| s.parse().unwrap()).collect() ; // why collect()
    apply_func(k)
}

fn main() {
    let input = std::fs::read_to_string("./src/0099_base_exp.txt").expect("file not found");
    println!("{:?}", type_name_of_val(&input));
    
    let parts: Vec<&str> = input.split("\n").collect(); // why collect()? it doesn't work without collect

    //let result: f64 = splits(&parts[0]);
    //println!("{:?}", result);
    let result:Vec<f64> = parts.iter().map(|c| splits(c)).collect() ;
    let value: usize = argmax(&result).expect("Not found") ;
    println!("{:?}", value);
    // [0] is very different so how do I pass .get() here? it doesn't work
    
}
