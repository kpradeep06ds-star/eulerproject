
pub fn is_palindrome(x:i32) -> bool{

    if x != 0 && x %10 == 0 {
        return false;
    }
    
    let mut revnum = 0;
    let mut xa = x;
    
    while xa > revnum{
        revnum = revnum*10 + xa % 10;
        xa = xa/10;
    }

    return xa == revnum || xa == revnum/10;

}

fn is_binary_palindrome(x: i32) -> bool{
    if x == 0{
        return true;
    }

    let n:i32 = x.ilog2() as i32 + 1 ;
    let mut left:i32 = 0;
    let mut right:i32 = n - 1;

    while left < right{
        if ((x >> left) & 1) != ((x >> right) & 1) {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true

}

fn main() {
    //println!("Hello, world!");
    // let x: i32 = 585;
    // let result1 : bool = is_binary_palindrome(x) ;
    // let result2: bool = is_palindrome(x);
    // println!("{}", result1 && result2);
    let mut count = 0;
    for i in 0..=1000000{
        if is_palindrome(i) && is_binary_palindrome(i){
            count += i;
        }
    }

    println!("{}", count);
}
