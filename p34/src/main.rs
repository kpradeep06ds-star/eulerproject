pub fn digits(num:i32) -> Vec<i32>{
    let mut rem:i32;
    let mut newnum:i32 = num;
    let mut v:Vec<i32> = Vec::new();
    loop{
        rem = newnum%10 ;
        newnum = newnum/10 ;
        v.push(rem);
        if newnum < 1{
            break;
        }
    }
    //v.reverse();
    v
}

fn main() {
    // This problem requires to understand the upper domain
    // 9 factorial is 362880, 9!9! (9! and 9! not 99!) factorial is = 362880x2 which means 
    // 7 digit numbers would be 2540160
    // Now question says 3 digit gives 3 digit number so if any factorial combination expands should be out of domain
    // so if we see 8 digits 10**8 - 1 would have 100_000_00 but for this 362880x8 = 2903040, so with even all digits are 9
    // the maximum possible number can't go beyond 2540160, so we can't go beyond 8 digits or beyond
    let ul:i32 = 2_540_160;
    let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let mut finalv:Vec<i32> = Vec::new();
    for i in 10..=ul{
        
        let v = digits(i);
        //println!("{:?}", v);

        let mut sums = 0  ;
        for k in v{
            sums += factorials[k as usize];
            
        }
        //println!("{:?},{:?}", sums, i);
        if sums == i{
            finalv.push(i);
        }

    }

    println!("{:?},{:?}",finalv, finalv.iter().sum::<i32>());
    
}
