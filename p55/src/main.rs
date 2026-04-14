pub fn reverse(x: u128) -> u128{
    let mut rev = 0;
    let mut xa = x;
    while xa > 0{
        rev = rev*10 + xa%10;
        xa = xa/10;
    }
    rev
}

pub fn is_palindrome(x:u128) -> bool{

    if x != 0 && x %10 == 0 {
        return false;
    }
    // eg:x = 12321
    // revnum = 1
    // x = 1232
    // revnum = 10 + 2
    // x = 123
    // revnum = 120 + 3 => exit => x = 12 and revnum = 123 : x == revnu/10 here
    // x = 12
    // revnum = 123 + 2
    // x = 1
    let mut revnum = 0;
    let mut xa = x;
    while xa > revnum{
        revnum = revnum*10 + xa % 10;
        xa = xa/10;
    }
    return xa == revnum || xa == revnum/10;

}

fn main() {
    let value_max = 10_000;
    let mut pcount = 0;
    let n = 50;
    for i in 0..=value_max{
        let mut c = i;
        let mut cntr = n;
        loop {
            c = reverse(c) + c;
            if is_palindrome(c){
                pcount += 1;
                break
            }
            if cntr <= 0 {
                break;
            }
            cntr -= 1;
        }
    
    }
    println!("{:?}", value_max + 1 - pcount);

}
