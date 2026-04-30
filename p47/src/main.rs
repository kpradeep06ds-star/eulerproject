//use std::collections::HashSet;

// fn is_prime(n: i64) -> bool {
//     if n < 2 { return false; }
//     if n == 2 || n == 3 { return true; }
//     if n % 2 == 0 || n % 3 == 0 { return false; }
//     let mut i = 5;
//     while i * i <= n {
//         if n % i == 0 || n % (i + 2) == 0 { return false; }
//         i += 6;
//     }
//     true
// }

// My earlier approach was slow and didn't work with 4 for some reason, 
// Unable to figure out what happened there
// Leaving it my failure for future self
fn main() {
    let limit = 150_000;
    let mut factors = vec![0u8; limit];


    for i in 2..limit {
        if factors[i] == 0 { // i is prime
            let mut multiple = i;
            while multiple < limit {
                factors[multiple] += 1;
                multiple += i;
            }
        }
    }

    let mut count = 0;
    for i in 2..limit {
        if factors[i] == 4 {
            count += 1;
            if count == 4 {
                println!("First number of the sequence: {}", i - 3);
                println!("Sequence: {}, {}, {}, {}", i - 3, i - 2, i - 1, i);
                break;
            }
        } else {
            count = 0;
        }
    }
}


// fn main() {
//     let mut ans:Vec<i64> = Vec::new();
//     let x:usize = 200_000;
//     let mut primes:HashSet<i64> = HashSet::new();
//     let mut v:Vec<i64> = vec![0;x];
//     for i in 2..x{
//         if !is_prime(i as i64){
//             for j in &primes {
//                 if (i as i64)%j == 0{
//                     v[i as usize] += 1;
//                     if v[i] == 4{
//                         ans.push(i as i64);
//                     }
//                 }
//             }
//         } else {
//             primes.insert(i as i64);
//         }

//     }
//     let mut i:usize = 0;
//     let mut j:usize = 1;
//     let mut diff ;
//     let mut count = 0;
//     // let mut fans=0;
//     for _ in 0..(ans.len()-1){
        
//         diff = ans[j] - ans[i];
//         i +=1 ;
//         j +=1 ;

//         if diff == 1 {
//             count += 1;
//             if count == 4{
//                 println!("this is the answer {} {} {} {}",ans[i-3], ans[i-2], ans[i-1], ans[i]);
//                 break;
//             }
//         } else {
//             count = 0;
//         }
        
//     }

//     //println!("{:?} {} {} {}", ans, i , j, ans[i]);
// }
