fn main() {

    let n:i32 = 1001;
    let mut tr:i32 ;// n*(4*n.pow(2) - 1)/3; // top right
    let mut tl:i32 ; // top left
    let mut dl:i32 ; // down left 
    let mut dr:i32 ; // down right
    let mut sums:i32 = 0;

    for i in (3..=n).step_by(2){
        tr = i.pow(2);
        tl = i.pow(2) - (i - 1) ;
        dl = tl - (i - 1);
        dr = dl - (i - 1);
        sums += tr + tl + dl + dr;

    }

    println!("{}", sums + 1);

}
