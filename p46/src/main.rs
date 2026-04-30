// reverse the operation of goldbach
// O - 2*m^2 and see if expression is negative 
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


fn expression(o:i64, m:i64) -> i64{
    o - 2*m.pow(2)
}

fn main() {
    let mut start = 9; // First odd composite
    loop {
        if start % 2 != 0 && !is_prime(start) {
            let mut found_prime_combination = false;
            
            for i in 1.. {
                let v = expression(start, i);
                
                if v <= 0 { 
                    break; 
                }
                
                if is_prime(v) {
                    found_prime_combination = true;
                    break; 
                }
            }
            

            if !found_prime_combination {
                println!("The smallest odd composite that fails: {}", start);
                break;
            }
        }
        start += 2; 
    }
}