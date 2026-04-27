use itertools::Itertools;


fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

// it is asking for largest pandigital number, I will start with 987654321, 
// this should be largest but whether it's a prime or not is to be seen,however 
// this is not prime because it's divisble by 9 (adding digits leads to 9 - divisibility test), 
// so I would now know that similarly I can elemenate 8 digit number as it is also 
// divisible by 9, Now we have 7 digits to verify so I will start with 7654321 , 
// from here I will shuffle and in such a way that last digit is not multiple of 2 or 5, 
// in 6 digit we also have no solution , same for 5 digit, so now 4 digit to be seen so 
// now solution is limited to 7 or 4 digit

fn digits(v:Vec<&i64>) -> i64{
    
    let mut digit:i64 = 0;
    for (idx, d) in v.iter().enumerate(){
        digit = digit + *d*10_i64.pow(idx as u32) ;
    }
    digit

}

fn the_prime(start: Vec<i64>) -> Vec<i64>{
    let len = start.len();
    let params = start.iter().permutations(len) ;
    let mut d ;
    let mut all_primes:Vec<i64> = Vec::new() ;
    for p in params{

        d = digits(p);
        if is_prime(d){
            all_primes.push(d)
        }
    }
    return all_primes; 
}

fn main() {
    //println!("Hello, world!");
    // 1...9 there would be 
    let start:Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];
    let v:Vec<i64> = the_prime(start);
    let max:i64 = *v.iter().max().unwrap();
    println!("{}", max);

}
