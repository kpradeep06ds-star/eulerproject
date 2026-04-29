use itertools::Itertools;

fn main() {

    let pandigital = vec![0,1,2,3,4,5,6,7,8,9];
    let perms = pandigital.iter().permutations(10);
    let mut total = 0;
    
    for i in perms{
        let num1 = i[7]*100+i[8]*10+i[9];
        let num2 = i[6]*100+i[7]*10+i[8];
        
        if *i[0] != 0 && *i[3]%2 == 0&& (i[2] + i[3] + i[4])%3 == 0 && *i[5]%5 == 0 && (i[4]*100 + i[5]*10 + i[6])%7 == 0 && (i[5]*100 + i[6]*10 + i[7])%11 ==0 && num1%17 == 0 && num2%13 == 0{
                let mut sumv=0;
                for (idx,k) in (0..=9).rev().enumerate(){
                    sumv += i[idx]*10_i64.pow(k);
                }
                total += sumv;
        }
    }
    println!("{:?}",total);

}
