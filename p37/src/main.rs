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

fn is_left_truncatable(mut n: i64) -> bool {
    
    let mut divisor = 10_i64.pow(n.ilog10());
    while n > 0 {
        if !is_prime(n) { return false; }
        n %= divisor;
        divisor /= 10;
    }
    true
}

fn main() {
    let mut candidates = vec![2, 3, 5, 7];
    let mut results = Vec::new();
    let middle_digits = [1, 3, 7, 9];
    
    let mut head = 0;
    // head < candidates.len() -> this can be used as well (this will work as there are no primes more than 11 that exists in the known sample space)
    // the moment it will hit 11 we are out of loop
    while results.len() != 11 {
        // candidates is interesting
        // why - because it is always prime and right trimmed prime
        // right trim means trimming from right hand towards left
        // left trim means trimming from left hadn towards right
        // so extending candidates makes sense we are adding right trimmed like 2,3,5,7
        // however we can pop also but not required
        // Two important learnings -> the left truncation and right truncation is not a 'AND' condition
        // one must be checked after the other is checked (this sounds like 'AND' but it's not)
        // because middle digit can contain 9 (I am taking this example because 9 is simple)
        // 397 is prime also when we trucate right we have all primes BUT,
        // on left 39 becomes a non prime if you completely remove these sequences, you won't get : 739397
        // when we check this together that 397 will cause a problem but once it is accomplished that it's a middle segment and left truncation would never reach 39 this must be taken
        let current = candidates[head];
        head += 1;

        for &digit in &middle_digits {
            let next_num = current * 10 + digit;
            
            if is_prime(next_num) {
                candidates.push(next_num);
                
                if is_left_truncatable(next_num) {
                    results.push(next_num);
                }
            }
        }
    }

    results.sort_unstable();
    println!("Primes: {:?}", results);
    println!("Sum: {}", results.iter().sum::<i64>());
}