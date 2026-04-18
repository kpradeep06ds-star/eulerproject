use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut v: Vec<BigUint> = vec![
        2u32.into(),
        1u32.into(),
        2u32.into(),
        1u32.into(),
        1u32.into(),
    ];

    for i in 5u32..=99 {
        if (i + 1) % 3 == 0 {
            v.push((((i + 1) / 3) * 2).into());
        } else {
            v.push(1u32.into());
    }
}

    let max = 99;
    let mut num: BigUint = v[max].clone();
    let mut denom: BigUint = BigUint::one();

    for i in (0..max).rev() {
        let new_num = v[i].clone() * num.clone() + denom.clone();
        let new_denom = num;

        denom = new_denom;
        num = new_num;
    }

    println!("{} {}", num, denom);

    let digit_sum: u32 = num
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum();

    println!("{}", digit_sum);

}