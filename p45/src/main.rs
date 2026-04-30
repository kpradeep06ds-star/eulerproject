// triangular = n*(n+1)/2
// hexagonal = m(2*m - 1)
// there is a relationship b/w tri and hexa
// set n = 2k - 1;
// T of (2k-1) = H of (k)
// loop from k = 144 to whatever
// Now solve pentagonal number by X = j(3*j - 1)/2
// j = (1 + sqrt(1 + 24X)) / 6
// since X = m(2*m - 1) so if we start with k = 144 then tri value of k = k(2*k -1) of hex
// which means X = k(2*k - 1)
// Now use the conditions above to explore the boundaries


fn all_equal(x:i64) -> i64{
    let mut hexvalue : i64;
    let mut temp :i64;
    let mut k:i64 = x; // this is the trivalue, we will start with 144 from question
    loop {
        hexvalue = k*(2*k - 1); // which is equal to hexvalue - via math
        temp = 1 + 24*hexvalue; // for validation of X
        let root = temp.isqrt() as i64;
        if root*root == temp{
            if (root + 1)% 6 == 0{ // X is validated here for any k this is the answer
                return hexvalue;
            }
        }
        k+=1 ;

    }
}

fn main() {
    
    let result = all_equal(144);
    println!("{}", result);

}
