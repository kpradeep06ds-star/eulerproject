pub fn fib(n:u32) -> u32{
    
    let n_f = n as f64;
    let sqrt_5 = 5.0_f64.sqrt();

    let phi = (1.0 + sqrt_5)/2.0;
    let psi = (1.0 - sqrt_5)/2.0;

    let s = (phi.powf(n_f) - psi.powf(n_f))/sqrt_5 ;
    s.round() as u32

}

pub fn is_even_fib(n: u32) -> bool{
    fib(n)%2 == 0
}
// the above two formula is not scalable
// a better apporach is that: use a while loop and keep adding the numbers and  removing the previous ones
// store the sums

fn main() {
    
    // let maxlimit = 10;
    // let minlimit = 2;

    // let total:u32 = (minlimit..=maxlimit).filter(|&c| is_even_fib(c)).sum();
    // println!("{:?}", total);


    let maxlimit = 4000000;
    let mut sums = 0;
    let mut a = 1;
    let mut b = 2;

    while b <= maxlimit{
        if b%2 == 0{
            sums += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    println!("{:?}", sums);
    println!("{:?}", optimized());
}

// Another anlyatical smarter formula
pub fn optimized() -> u64{
    let limit = 4_000_000;
    let mut sum = 0;

    let mut a = 2;
    let mut b = 8;

    while a <= limit {
        sum += a;
        let next = 4 * b + a;
        a = b;
        b = next;
    }

    sum as u64
}