fn split_digits(x:i32) -> Vec<i32>{
    let r:Vec<i32> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
    r
}

fn uncommon(v1:Vec<i32> , v2:Vec<i32>) -> Vec<i32> {
    for (id, i) in v1.iter().enumerate(){
        for (jd, j) in v2.iter().enumerate(){
            if i == j{
                return vec![v1[1 - id], v2[1 - jd]];
            }
        }
    }
    return vec![-1, -1];
}

fn product(v:Vec<Vec<i32>>) -> Vec<i32>{
    let mut top =1;
    let mut down=1;

    for vi in v{
        top = top*vi[0];
        down = down*vi[1];
    }
    vec![top, down]
}

fn gcd(mut a:i32, mut b:i32) -> i32{
    while a != b{
        if a > b{
            a = a - b;
        } else {
            b = b - a;
        }
    }
    return a;
}

fn main() {
    let mut n:Vec<i32>;
    let mut d:Vec<i32>;
    let mut pairs: Vec<Vec<i32>> = vec![];
    let mut uc:Vec<i32>;
    for num in 11..=99{
        for denom in (num+1)..=98{
            if num < denom && num%10 != 0 && denom%10 != 0{
                n = split_digits(num);
                d = split_digits(denom);
                uc = uncommon(n, d);
                if !uc.contains(&-1){
                    if uc[0]*denom == uc[1]*num{
                        pairs.push(vec![num, denom]);
                    }
                }

            }
        }
    }
    let p = product(pairs.clone());
    let g = gcd(p[0], p[1]);
    println!("{:?} {:?} {:?} {:?}",&pairs, g, p, p[1]/g);

}
