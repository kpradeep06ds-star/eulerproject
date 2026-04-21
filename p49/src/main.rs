use itertools::Itertools;

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
    let mut vprime: Vec<i64> = vec![1009];

    for i in (1011..targetnum).step_by(2) {
        if is_prime(i) {
            vprime.push(i);
        }
    }
    vprime
}

pub fn digits_split(mut n: i64) -> Vec<i64> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

fn group_primes(x: i64, v: &[i64]) -> Vec<i64> {
    let digits: Vec<i64> = digits_split(x);
    let mut nums: Vec<i64> = Vec::new();

    for p in digits.iter().permutations(digits.len()) {
        let num = p[0] * 1000 + p[1] * 100 + p[2] * 10 + p[3];
        if v.contains(&num) && !nums.contains(&num) {
            nums.push(num);
        }
    }

    nums.sort();
    nums
}

fn main() {
    let v: Vec<i64> = all_primes(9999);

    for vp in &v {
        let temp = group_primes(*vp, &v);

        if temp.len() >= 3 {
            for i in 0..temp.len() {
                for j in i + 1..temp.len() {
                    let third = 2 * temp[j] - temp[i];
                    if temp.contains(&third) {
                        if temp[i] == 1487 && temp[j] == 4817 && third == 8147 {
                            continue;
                        }

                        println!("{}, {}, {}", temp[i], temp[j], third);
                        println!("{}{}{}", temp[i], temp[j], third);
                        return;
                    }
                }
            }
        }
    }
}