use std::collections::HashMap;

// a + b > c
// a**2 + b**2 = c**2
// all a, b, and c must be integer
// p = a + b + c
// c = (p - a - b)
// b = p**2 - 2*p*a / 2*p - 2*a
fn anlytical_p(p:i64, a:i64) -> i64 {
    let rem = (p.pow(2) - 2*p*a) % (2*(p - a));

    if rem == 0{
        return  (p.pow(2) - 2*p*a) / (2*(p - a));
    } else {
        return 0;
    }
   // this suggest that 1/2 of anything and since every a, b, c should be integer
}
// the divion with 1/2 on right means p**2 - 2*p*a/(p - a) must be integer 
// this formula suggest if p is odd then entire expression would be odd
// if p is even then entire expression is even
// Now can we determine if either of a or b is even 
// with trial and error we can...but I will write a math proof I got it

// a + b  > c
// suggests that one of the side is larger than 1/3 in case right angle triangle and it's not equilateral triangle
// so the search space must p/3 for one of the sides

fn main() {
    let mut b:i64;
    let mut v:Vec<(i64, i64, i64)> = Vec::new();
    let mut c: i64 ;

    for p in (12..=1000).step_by(2){
        // one of them must be less than p/3 we established that
        for a in 1..=p/3{
            b = anlytical_p(p, a);
            //println!("{}, {}, {}",a, b, p);
            if b != 0 && b > a{
                c = (a.pow(2) + b.pow(2)).isqrt();
                if p == a + b +c {
                    v.push((a, b, c));
                }
            }
        }
    }
    
    let mut hmap:HashMap<i64, i64> = HashMap::new();

    for h in v{
        let sums = h.0 + h.1 + h.2;
        *hmap.entry(sums).or_insert(0) += 1 ;
    }

    let mut score_vec: Vec<(&i64, &i64)> = hmap.iter().collect();
    score_vec.sort_by(|a, b| a.1.cmp(b.1));

    println!("{:?}", score_vec[score_vec.len()-1]);


}
