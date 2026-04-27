use std::collections::HashMap;

fn digit_explosion(x:i64) -> Vec<i64>{
    let mut y = x;
    let mut rem ;
    let mut v:Vec<i64> = Vec::new();
    while y >= 1{
        rem = y%10; 
        v.push(rem);
        y = y/10;
        
    }
    v.reverse();
    v
}


fn analytical_sol(x:i64) -> HashMap<i64, i64>{
    let mut hmap:HashMap<i64, i64> = HashMap::new();
    let mut sums = 0;
    for a in 1..=x{
        sums = 9*a*10_i64.pow((a-1).try_into().unwrap()) + sums; 
        hmap.insert(a, sums); // cumulative
    }
    
    hmap

}


fn logic(x:i64, hmap:HashMap<i64, i64>) -> i64{

    let mut diff:Vec<i64> = Vec::new();
    let inverted: HashMap<i64, i64> = hmap.clone().into_iter().map(|(k, v)| (v, k)).collect();
    if x < 10{
        return x;
    }
    
    for (_, v) in &hmap{
        if v < &x {
            diff.push(*v);
        }
    }
    let mapvalue = diff.iter().max().unwrap();
    let skip = x - mapvalue -1;
    let howmany_digit_x = inverted.get(mapvalue).unwrap() + 1;
    let value = skip/howmany_digit_x;
    let rem = skip % howmany_digit_x;
    let add_digit = 10_i64.pow(howmany_digit_x as u32 - 1) + value;
    let move_digit = digit_explosion(add_digit);
    move_digit[rem as usize]
}

fn main() {
    let x = 7;
    let result = analytical_sol(x);
    let mut mul = 1;
    for i in [1, 10, 100, 1000, 10000, 100000, 1000000]{
        let a = logic(i, result.clone());
        mul = mul*a;
    }
    println!("{:?}", mul);


}
