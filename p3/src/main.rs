pub fn is_prime(n: i64) -> bool{
    if n < 2{
        return false;
    } else if n == 2{
        return true;
    } else {
        for i in 2..=n.isqrt(){
            if n%i == 0{
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let targetnum: i64  = 600851475143;
    let num: i64 = targetnum.isqrt();
    let mut start = num ;

    if start%2 == 0{
        start -= 1;
    }
    // println!("{}", num);
    loop {
        // println!("{:?}", start);
        // println!("{:?}", targetnum % start == 0);
        // println!("{:?}", is_prime(targetnum));
        if targetnum % start == 0 && is_prime(start){
            break;
        } else if start <= 1{
            break;
        }
        start -= 1;
    }
    println!("{:?}", start);
}
