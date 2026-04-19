use std::collections::HashSet;

// pub fn log_any_base(number: f64, base: f64) -> f64 {
//     number.ln() / base.ln()
// }

//
// We are factorizing once, then scaling exponents for every b, building a vector representation of the number, 
// and inserting that into a HashSet
// If two expressions result in the same prime - exponent structure, HashSet removes duplicates 
// so uniqueness is achieved without computing the actual large numbers.
// (prime, count)
// increase or rather multiply the count in .map().collect() and try adding in hashset if already there won't be inserted
// .map and collect() are key here...

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            factors.push((p, count));
        }
        p += 1;
    }

    if n > 1 {
        factors.push((n, 1));
    }

    factors
}

fn main() {
    let max = 100usize;
    let mut hset: HashSet<Vec<(usize, usize)>> = HashSet::new();

    for a in 2..=max {
        let factors = factorize(a);

        for b in 2..=max {
            let repr: Vec<(usize, usize)> = factors
                .iter()
                .map(|&(p, e)| (p, e * b))
                .collect();

            hset.insert(repr);
        }
    }

    println!("{}", hset.len());
}