
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
    let amin:i64 = -999;
    let amax:i64 = 999;
    let bmin:i64 = 3;
    let bmax:i64 = 999;
    let nmax:i64 = 1000;

    let mut tuple:(i64, i64, i64, i64) = (0, 0, 0, 0) ;
    let mut count;
    for a in (amin..=amax).step_by(2){
        
        for b in (bmin..=bmax).step_by(2){
            count = 0;
            if is_prime(b){
                if is_prime(1 + a + b){
                    for n in 0..=nmax{
                        let result = n.pow(2) + a*n + b;
                        if is_prime(result){
                            count += 1;
                            if count >= tuple.2{
                            tuple = (a, b, count, a*b);
                            }
                        } else{
                            break;
                        }
                         
                    }
                }
            }
        }
    }
    println!("{:?}", tuple);
}
