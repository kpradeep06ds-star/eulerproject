fn multiply(x:i32) -> i32 {
    if (x % 3 == 0) && (x % 5 != 0){
        return x;
    } else if (x % 3 != 0) && (x % 5 == 0){
        return x;
    } else if (x % 3 == 0) && (x % 5 == 0){
        return x;
    }
    0
}

// fn main() {

//     let result:i32 = (1..1000).map(|c| multiply(c)).sum();
//     println!("{:?}", result);
// }

// alternate

fn main() {
    // we can also solve this using AP formula
    // n/2 * {2*a - (n-1)*d} => n/2*{2*3 + (n-1)*3} => n/2*(3n + 3)
    let result: i32 = (1..1000)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum();

    println!("{}", result);
}