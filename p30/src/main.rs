
pub fn digits(mut n:i64) -> Vec<i64>{
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse(); 
    digits
}

// pub fn upper_bound(x:f64){
//     let pow:f64 = (9 as f64).powf(5.0);
//     let m:f64 = (x as f64)*(pow);
//     let length_digit:f64 = (x as i64).ilog().round(0) + 1;

//     let upper_limit:f64 = length_digit*m;
//     if upper_limit >= 10.pow(length_digit){
//         return upper_limit;
//     } else {
//         return (x - 1)*(pow);
//     }
// }

fn main() {
    let mut d:Vec<i64>;
    let n:i64 = 5;
    let mut t:i64;
    let mut v:Vec<i64> = Vec::new();
    let ub: i64 = 354294 ; // this is not a magic number
    // this is the missing peace in the problem
    // how to determine this peace?
    // For a single digit: the maximum value is 9 against the 5th power is sum of 9^5
    // For 2 digit: sum of 2*9^5
    // we keep doing this
    // at 6 digit = sum of 6*9^5 = 354294 and this number is still larger than 100_000
    // at 7 digit = sum of 7*9^5 = 413343 but this number is less than 1000_000 // hence we stop here and pick the number before this step
    
    for i in 1..=ub{
        d = digits(i);
        t = d.iter().map(|x:&i64| (*x as f64).powf(n as f64)).sum::<f64>() as i64;
        if t == i{
            v.push(i);
        }
    }
    println!("{:?}", v);
    println!("{:?}", v.into_iter().sum::<i64>() - 1);

}
