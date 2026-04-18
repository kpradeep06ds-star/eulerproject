//use std::collections::HashMap;

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

pub fn squares(x: Vec<i32>) -> i32{
    let sq:i32 = x.iter().map(|c| c.pow(2)).sum::<i32>();
    sq
}

pub fn find_breaks(x:i32) -> bool{
    if x == 89 || x == 1{
        return true;
    }
    return false;
}



fn main() {
    //let num: i32 = 1000_0000;
    //let mut hmap: HashMap<i32, i32> = HashMap::new() ;
    let mut d:i32 ;
    let mut v:i32 = 0;

    for i in 2..1000_0000{
        d = i;
        loop{
            d = squares(digits(d));
            //*hmap.entry(i).or_insert(0) += 1;
            if find_breaks(d){
                if d == 89{
                    v += 1;
                }
                break;
            }
        }
    }

    println!("{:?}", v)  ;  
}
