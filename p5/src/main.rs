pub fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn get_lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 { return 0; }
    // We divide before multiplying to prevent potential integer overflow
    (a / get_gcd(a, b)) * b
}

pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else {
        for i in 2..=n.isqrt() {
            if n % i == 0 {
                return false;
            }
        }
    }

    true
}

fn main() {
    // let mut value:i64 = 1;
    // let mut v:Vec<i64> = Vec::new();
    // for i in 1..=20{
    //     if is_prime(i){
    //         value*=i;
    //     } else{
    //         v.push(i);
    //     }
    // }



    //multiply all the primes first then find the 
    // filter out the primes under 20
    // 2, 3, 5, 7, 11, 13, 17, 19
    // To achieve 20 = 2x5x2 
    // To achieve 18 = 2 x3 x3
    // To achieve 15 = 3 x 5
    // Toachieve 14 = 2x7
    // to achieve 12 = 3 x 2 x 2
    // to achieve 10 = 2 x 5
    // to achieve 9 = 3 x 3
    // to achieve 8 = 2 x 2 x 2
    // to achieve 6 = 3 x 2
    // to achieve  4 = 2 x 2
    // find the common mulitple in all these as primes : 2 x 2 x 2 x 3= 8x3 = 24
    let mut result = 1;

    for i in 1..=20 {
        result = get_lcm(result, i);
    }
    println!("{} {}" ,value, result);
}
