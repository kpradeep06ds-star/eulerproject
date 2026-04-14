use std::any::type_name_of_val;


fn main() {
    let input = std::fs::read_to_string("./src/0089_roman.txt").expect("file not found");
    println!("{:?}", type_name_of_val(&input));
    // this is a result object interesting: "core::result::Result<alloc::string::String, std::io::error::Error>"
    // what does expect do here? 
    // In case option is None or Result is Err then .expect() gives a panic! 
    // IN case of option is Some or Result is Ok then .expect() extracts the information
    let original_length = input.chars().filter(|c| !c.is_whitespace()).count();
    // println!("{:?}", original_length);    
    // rust whitespace is very broad it also ignores \n here
    let roman_new = input.replace("DCCCC", "CM").replace("CCCC", "CD").replace("LXXXX", "XC").replace("XXXX", "XL").replace("VIIII", "IX").replace("IIII", "IV");
    let modified_length = roman_new.chars().filter(|c| !c.is_whitespace()).count();

    println!("{:?}", original_length - modified_length);

}
