
pub fn count(k:i32) -> i32 {
    // 200, 100, 50,20,10,5, 2, 1

    let coins:Vec<i32> = vec![1,2,5,10,20,50,100,200] ;
    let target = k;
    let mut ways  = [0; 201]; // Note we start from 0 to 200 hence 201 space in vector
    // two loops one for all the coins
    ways[0] = 1; // set this to 1 , there is only one way we can get 0 pound
    for coin in coins{
        // this ensures that i should be capped so for example
        // if i  = 10, then loop would go from 10 to 200
        // which means i - coin will always be >= 0
        for i in coin..=target{
            ways[i as usize] = ways[i as usize] + ways[i as usize - coin as usize];
        }
    }
    
    ways[k as usize]
}

pub fn main(){
    let value = count(200);
    println!("{:?}", value);
}