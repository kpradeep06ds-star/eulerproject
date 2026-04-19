// use std::any::type_name_of_val;


// 1. Pick all the words
// 2. Sort the alphabets
// 3. Find which are anagram -> write a function which sees two words if they are anagram
// 4. create a hash table: with sorted alphabets as  keys and values would contain vector of same anagrams
// 5. Make a list of all the squares by squaring every number until maximum length - 16 alphabets of given alaphabets
// 6. 10**8 the maximum so I need to find out all the numbers until this value
// 7. Now I will test each of these with all 10 numbers except 0 at begining
// 8. How to make this search -> for each hash value pick numbers from step 7 so if a word is ABCD then there are  9*9*8*7 possibilites but 
// I don't have to do it, I will look into the squares numbers and look for length of 4 and search in that




// pub fn all_squares(length: u32) -> Vec<i64> {

//     let lst_elem = 10_i64.pow(length.isqrt() + 1);
//     let mut squares: Vec<i64> = Vec::new();
//     let mut i:i64 = 0 ;
//     loop {

//         i += 1;
//         squares.push(i.pow(2));
//         if i >= lst_elem{
//             break
//         }
//     }
//     squares
// }

pub fn get_squares_by_len(max_len: usize) -> std::collections::HashMap<usize, Vec<i64>> {
    let mut map: std::collections::HashMap<usize, Vec<i64>> = std::collections::HashMap::new();
    let limit = 10_i64.pow(max_len as u32);
    
    let mut i = 1_i64;
    while let Some(sq) = i.checked_mul(i) {
        if sq >= limit { break; }
        let len = sq.to_string().len();
        map.entry(len).or_insert(Vec::new()).push(sq);
        i += 1;
    }
    map
}


pub fn is_anagram(a: &str, b: &str) -> bool {
    if a.len() != b.len(){
        return false;
    }
    let mut a_chars: Vec<char> = a.chars().collect() ; // what kind of object sequence of chars but what kind?
    let mut b_chars: Vec<char> = b.chars().collect();

    a_chars.sort();
    b_chars.sort();

    for (idx, k) in a_chars.iter().enumerate(){
        if *k != b_chars[idx]{
            return false;
        }
    }
    return true;

}


fn main() {
    // why expect?
    let line = std::fs::read_to_string(r"./src/0098_words.txt").expect("file not found");
    //println!("{:?}", type_name_of_val(&line));
    let parts: Vec<&str> = line.split(",").collect();
    //println!("{}", parts[1]);
    let size: usize = parts.iter().map(|c| c.len()).max().expect("there is something wrong") ;
    let squares = get_squares_by_len((size as u32).try_into().unwrap());
    let mut length: usize;
    for p in parts{
        length = p.len();
        squares[&length];
        todo!("this is to be written");
    }

    //println!("{:?}", &squares[&2][0]);
    
    
    //let squares: Vec<i64> = all_squares(size as i64);
    //let v = vec!["SPORT", "TOPSR"] ;
    //let b : bool = is_anagram(&v[0], &v[1]);
    //println!("{} {}", size, b);

}
