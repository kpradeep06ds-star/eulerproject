// No loops please
// analytical solution

// sum of n natural numbers

fn sum_n(i:i64) -> i64{
    i*(i+1)/2
}

fn sum_nsq(n:i64) -> i64{
    (n*(n+1)*(2*n + 1))/6 // please never do (1/6) * (n + 1) * (2*n + 1) *n , the 1/6 will be zero??? ahhh..
}

fn main() {
    let n = 100;
    let sum_n_value = sum_n(n);
    let sum_nsq_value = sum_nsq(n);

    let result = sum_n_value.pow(2) - sum_nsq_value ;
    println!("{}", result);
}
