fn split_digits(x: i32) -> (i32, i32) {
    (x / 10, x % 10)
}

fn uncommon(n: (i32, i32), d: (i32, i32)) -> Option<(i32, i32)> {
    let (n1, n2) = n;
    let (d1, d2) = d;

    if n1 == d1 {
        Some((n2, d2))
    } else if n1 == d2 {
        Some((n2, d1))
    } else if n2 == d1 {
        Some((n1, d2))
    } else if n2 == d2 {
        Some((n1, d1))
    } else {
        None
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

fn main() {
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    let mut top = 1;
    let mut down = 1;

    for num in 11..=99 {
        for denom in (num + 1)..=99 {
            if num % 10 == 0 || denom % 10 == 0 {
                continue;
            }

            let n_digits = split_digits(num);
            let d_digits = split_digits(denom);

            if let Some((new_num, new_denom)) = uncommon(n_digits, d_digits) {
                if new_denom != 0 && new_num * denom == new_denom * num {
                    pairs.push((num, denom));
                    top *= num;
                    down *= denom;
                }
            }
        }
    }

    let g = gcd(top, down);

    println!("pairs = {:?}", pairs);
    println!("product = {}/{}", top, down);
    println!("reduced = {}/{}", top / g, down / g);
    println!("answer = {}", down / g);
}