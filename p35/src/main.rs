// use std::collections::HashSet;

pub fn is_prime(n: i64) -> bool{
    if n < 2{
        return false;
    } else if n == 2{
        return true;
    } else {
        for i in 2..=n.isqrt(){
            if n%i == 0{
                return false;
            }
        }
    }

    return true;
}

fn digits(x:i64) -> Vec<i64>{
    let v:Vec<i64> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
    v
}

fn combine_digits(v:Vec<i64>) -> i64{
    //v.sort_unstable();
    let mut num = 0;
    for (idx, k) in v.into_iter().rev().enumerate(){
        num += k*(10_i32.pow(idx.try_into().unwrap()) as i64);
    }
    num 
}

fn next_cycle(v:Vec<i64>) -> Vec<i64>{

    let mut first = 0;
    let mut nv:Vec<i64> = Vec::new();
    for (idx, m) in v.into_iter().enumerate(){
        if idx == 0{
            first = m;
        } else{
            nv.push(m);
        }
    }
    nv.push(first);
    nv
}

fn sorted(d:i64) -> (Vec<i64> , i64){
    let v = digits(d);

    for i in &v{
        if i%2 == 0 || i%5 == 0 {
            return (vec![], 0);
        }
    }

    let mut m = v.clone();
    let mut ac ;
    let mut all_cyclic:Vec<i64> = Vec::new();
    for _ in 0..v.len(){
        m = next_cycle(m);
        ac = combine_digits(m.clone());
        if !is_prime(ac){
            return (vec![], 0);
        } else {
            all_cyclic.push(ac);
        }
    }
    (all_cyclic.clone(), all_cyclic.len() as i64)

    // v.sort_unstable();
    // let mut num = 0;
    // for (idx, k) in v.into_iter().enumerate(){
    //     num += k*(10_i32.pow(idx.try_into().unwrap()) as i64);
    // }
    // num 
}



fn main() {
    let v:Vec<i64> = (11..=1000000).step_by(2).filter(|c| is_prime(*c as i64)).collect() ;
    // all primes are correct
    // digits are splitted correctly
    // Now I have to create hashmap with sorted primes digits
    // 
    let mut lookup: Vec<i64> = Vec::new();
    let mut answer: Vec<i64> = Vec::new();
    let mut tup: (Vec<i64>, i64);
    for k in &v{
        if !lookup.contains(&k){
            tup = sorted(*k);
            lookup.extend(tup.0);
            answer.push(tup.1);
        } else {
            answer.push(1);
        }
    }
    // let hmap:HashMap<i64,Vec<i64>> = HashMap::new();
    
    // for h in &v{
    //     let d = sorted(*h);
    //     hmap.entry()
    // }
    
    //let k = v.get(1000).unwrap() ;
    // let result = sorted(*k);
    let single_digit_cylic = 4;
    //println!("{:?}", answer);
    println!("{:?}", single_digit_cylic + answer.iter().map(|c| if *c != 0 {1} else {0}).sum::<i64>());
}
