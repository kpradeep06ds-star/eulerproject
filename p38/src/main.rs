
// generate all 4 numbers
// multiply them with 1, 2
// 918273645 -> we want to beat this
// first number must be 9

fn digits(x:i64) -> i64{
    let mut y = x;
    let mut v:Vec<i64> = Vec::new() ;
    while y >= 1 {
        v.push(y % 10);
        y = y/10;
    }
    v.sort_unstable();
    let mut count = 0;
    let mut num = 0;
    for i in v.iter().rev(){
        num += i*10_i64.pow(count);
        count += 1;
    }
    num as i64
}

fn main() {

    let mut x;
    let mut v:Vec<i64> = Vec::new();
    //println!("{:?}", digits(69134));
    // why loop from 1001 to 9999 because 2 and 3 digit number's won't cut it
    // here is why - if use 1,2,3 as sequence 2 digit number we get 8 number digit with 1,2,3,4 then we get 11
    // same goes for 3 digit number
    // use 4 digit number , 5 digit and above can result to large numbers
    // Now a better optimization is to use 1 to 2 and not 1 to 9 
    // we already established that multiplication of 1 and 2 enough to give 9 digits , with other numbers digit count will rise
    // Infact we don't have to start from 1001 -> since we already know it will start from 9
    // so the begining must start from 9
    for i in 9001..=9999{

            x = (i * 100_000) + (i * 2);

            if x / 1000_000_00 != 9 || x % 9 != 0 || digits(x) != 123456789 {
                continue;
            } else {
                if !v.contains(&x){
                    v.push(x);
                }
            }
    }
    println!("{:?}", v);
}


