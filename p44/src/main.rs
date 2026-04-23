fn pentagonal(n:i64) -> i64{
    n*(3*n - 1)/2
}

fn is_pentagonal(x: i64) -> bool {
    let disc = 1 + 24 * x;
    let root = (disc as f64).sqrt() as i64;
    root * root == disc && (1 + root) % 6 == 0
}

fn main() {
    let mut dbest = i64::MAX;
    // my earlier attempt was with loop{} => but I crashed my system
    // So fixed it using for loop with  iterations known
    // if I don't find the output in search space I will increase the value of for loop
    for k in 2..10000 {
        let pk = pentagonal(k);

        // this is heuristic -> i looping in reverse for better differencing
        for j in (1..k).rev() {
            let pj = pentagonal(j);
            let d = pk - pj;

            if d >= dbest {
                break;
            }

            let s = pk + pj;

            if is_pentagonal(d) && is_pentagonal(s) {
                dbest = d;
                println!("D = {}, Pk = {}, Pj = {}", d, pk, pj);
            }
        }
    }

    println!("Best difference = {}", dbest);
}
