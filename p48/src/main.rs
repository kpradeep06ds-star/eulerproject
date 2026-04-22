// fn factorize(mut n: usize) -> Vec<(usize, usize)> {
//     let mut factors = Vec::new();
//     let mut p = 2;

//     while p * p <= n {
//         if n % p == 0 {
//             let mut count = 0;
//             while n % p == 0 {
//                 n /= p;
//                 count += 1;
//             }
//             factors.push((p, count));
//         }
//         p += 1;
//     }

//     if n > 1 {
//         factors.push((n, 1));
//     }

//     factors
// }

// fn modby(num:i64, powv:i64) -> i64{
//     let mut result:i64 = 1;

//     for _i in 1..=pow{
//         result = (result*num) % 10.pow(powv);
//     }
//     result
// }

fn modby(num: u128, powv: u128) -> u128 {
    let modulus = 10u128.pow(10); // fixed: last 10 digits
    let mut result = 1;

    for _ in 0..powv {
        result = (result * num) % modulus;
    }

    result
}

fn main(){
    let mut sums: u128 = 0;
    for i in 1..=1000{
        sums += modby(i as u128, i as u128);
    }
    //let result = modby(67, 67) ;
    println!("{}", sums % 10_u128.pow(10));
 
}
