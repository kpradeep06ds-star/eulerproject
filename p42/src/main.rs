use std::fs;
use std::collections::HashMap;

fn triangle_num(i:i64) -> i64{
    i*(i+1)/2
}

fn mapping_english() -> HashMap<char, i64>{
    let mut hmap:HashMap<char, i64> = HashMap::new();
    let mut count = 1;
    for c in 'A'..='Z' {
        hmap.insert(c, count);
        count += 1;
    }

    hmap
}

fn frequency(fields:Vec<&str>) -> HashMap<i64, i64> {
    let  hmap:HashMap<char,i64> = mapping_english();
    //println!("{:?}", hmap);
    let mut counter:HashMap<i64, i64> = HashMap::new();
    let mut value;
    for i in fields{
        value = 0;
        for ch in i.chars(){
            value += hmap.get(&ch).unwrap();
        }
        *counter.entry(value).or_insert(0) += 1;
    }

    counter

}


fn main(){

    
let content = fs::read_to_string("./src/0042_words.txt").expect("Could not read file");
let fields: Vec<&str> = content
    .split(',')
    .map(|s| s.trim_matches('"')) // This removes the surrounding quotes
    .collect();
// Rust is berzerk sometimes, I had to remove double quotes too !!!!
let result = frequency(fields);
let maxval = result.keys().max().unwrap();
let mut trival:Vec<i64> = Vec::new();
let mut count = 1;
let mut i = 1;

while i <= *maxval{
    i = triangle_num(count);
    trival.push(i);
    count += 1;
}

let mut tot_ans:i64 = 0;

for (idx, val) in result{
    if trival.contains(&idx){
        tot_ans += val;
    }
}

println!("{}", tot_ans);

}
