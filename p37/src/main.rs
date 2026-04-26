// The right truncation should have worked but not working, 
// if you look at tempnum that checks two things prime getting created while reversing the input and checking each element in between that is left truncation in action but if I am using the correct sequence shouldn't the right truncation already work? or should I write another for loop in non reverse way in the same way , i can do that,
// but I don't want to re - explode the digit? 

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


fn left_truncate_prime(v:Vec<i64>) -> i64{

    let mut num:i64 = 0;
    let mut count = 0;

    for i in v.into_iter().rev(){
        num += i*(10_u32).pow(count) as i64;
        
        if !is_prime(num){
            return 0;
        }
        count += 1;
        
    }
    if is_prime(num){
        return num;
    }
    return 0;
}


// fn digits(x:i64) -> Vec<i64>{
//     let v:Vec<i64> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
//     v
// }

fn right_truncate_prime(x:i64) -> bool{
    let mut y = x;
    if !is_prime(y){
        return false;
    }
    if y/10 < 1{
        return true;
    } else {
        loop {
            y = y/10;
            if y < 1{
                break;
            }
            
            if !is_prime(y){
                return false;
            }

        }
    }
    return true;
}

fn common_loop(s:Vec<i64>) -> Vec<i64> {
    
    let mut v:Vec<i64> = Vec::new();

    let tempnum = left_truncate_prime(s);
    if tempnum != 0  {
        if !v.contains(&tempnum) {
            v.push(tempnum);
        }
    } 
    v
}


fn main() {

    // these below edge cases we have to figure out by pen paper
    // starting can be any single digit which are prime that is why 2 3 5 7
    // middle can't multiple of 2 as truncation can easily led us to non prime
    // middle can't be also 5 
    // last is interesting except 3 and 7 no one can come - because 221 from left lead to 21 which is not prime 
    // same we search many examples and realise that last can be only 3 or 7
    let s:Vec<i64> = vec![2, 3, 5, 7];
    let m:Vec<i64> = vec![1, 3, 7, 9];
    let l:Vec<i64> = vec![3, 7];

    let mut v:Vec<i64> = Vec::new();

    for s1 in s.clone(){
        for s2 in m.clone(){
            v.extend(&common_loop(vec![s1, s2]));
            for s3 in m.clone(){
                v.extend(& common_loop(vec![s1, s2, s3]));
                for s4 in m.clone(){
                    v.extend(& common_loop(vec![s1, s2, s3, s4]));
                    for s5 in m.clone(){
                        v.extend(& common_loop(vec![s1, s2, s3,s4, s5]));
                    for s6 in l.clone(){
                        v.extend(& common_loop(vec![s1, s2, s3, s4, s5, s6]));
                        }
                    }
                }
            }
        }
    }

    // let vtruncate:Vec<i64> = vec![337, 397, 773, 2113, 2137, 2797, 
    // 3313, 3373, 5113,5197, 6197, 7937,79337, 79397,279397, 21997, 29137, 29173, 33797, 39397, 279337,
    // 291373,  291997, 299137, 379397, 391373, 399137, 399173, 513313,];
    let mut vf:Vec<i64> = vec![];
    for k in &v{
        if right_truncate_prime(*k){
            vf.push(*k);
        }
    }
    //println!("{:?}", right_truncate_prime(748317));
    // for i in &v{
    //     if !vtruncate.contains(&i){
    //         vf.push(*i);
    //     }
    // }

    vf.sort_unstable();
    //println!("{:?}", vf);
    println!("{:?} {:?}",vf, vf[0..11].iter().sum::<i64>());
}


// fn digits(x:i64) -> Vec<i64>{
//     let v:Vec<i64> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
//     v
// }

// fn to_digit_from_index(v:Vec<i64>, index:usize) -> usize {
//     // let mut newln = v.len() - index;
//     let mut count = 0;
//     let mut newnum = 0 ;
//     if index < v.len(){
//         for k in v[index..v.len()].into_iter().rev(){
//             newnum += (*k)*10_i64.pow(count);
//             if !is_prime(newnum){
//                 return 0;
//             }
//             count += 1;
//         }
//     } else {
//         return 0;
//     }
//     newnum as usize
// }

// 3, 7, 9, 3
//v = [3, 9, 7, 3] -> this is v
// vrev = [3, 7, 9, 3]
//3 + 