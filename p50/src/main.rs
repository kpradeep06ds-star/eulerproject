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

fn all_primes(targetnum: i64) -> Vec<i64> {
    let mut vprime: Vec<i64> = vec![2];

    for i in (3..targetnum).step_by(2) {
        if is_prime(i) {
            vprime.push(i);
        }
    }
    vprime
}


fn main() {
    let targetnum = 1_000_000;
    let v_all_primes: Vec<i64> = all_primes(targetnum);

    let mut vprefix_sum: Vec<i64> = vec![0];
    for i in v_all_primes.iter() {
        let next = vprefix_sum.last().unwrap() + *i;
        vprefix_sum.push(next);
    }

    let mut prime_lookup = vec![false; targetnum as usize];
    for &p in &v_all_primes {
        prime_lookup[p as usize] = true;
    }

    let n = vprefix_sum.len();
    let mut maxv = 0;
    let mut sumf = 0;

    for i in 0..n {
        for j in i + 1..n {
            let sum = vprefix_sum[j] - vprefix_sum[i];
            if sum >= targetnum {
                break;
            }

            let check_length = j - i;
            if prime_lookup[sum as usize] && check_length > maxv {
                maxv = check_length;
                sumf = sum;
            }
        }
    }

    println!("{} {}", maxv, sumf);
}