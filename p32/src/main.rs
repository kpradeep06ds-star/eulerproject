fn check(a: i32, b: i32) -> (i32, bool) {
    let fvalue = a * b;
    let f = format!("{}{}{}", a, b, fvalue);

    // 1. Must be exactly 9 characters
    // 2. Must NOT contain '0'
    if f.len() != 9 || f.contains('0') {
        return (fvalue, false);
    }

    let mut seen_digits = Vec::new();

    for k in f.chars() {
        if seen_digits.contains(&k) {
            return (fvalue, false); // Found a duplicate digit
        } else {
            seen_digits.push(k);
        }
    }
    (fvalue, true)
}

fn main() {
    let mut products: Vec<i32> = vec![];
    let mut temp:(i32, bool) ;

    for i in 1..=9{
        for j in 1234..=9876{
            temp = check(i as i32, j as i32);
            if temp.1 == true{
                products.push(temp.0);
            }
        }
    }

    for i in 12..=98{
        for j in 123..=987{
            temp = check(i as i32, j as i32);
            if temp.1 == true{
                products.push(temp.0);
            }
        }
    } 

    products.sort();
    products.dedup(); 
    
    let total: i32 = products.iter().sum();
    
    if let Some(&first) = products.first() {
        println!("Total Sum: {}, First Product: {}", total, first);
    } else {
        println!("No products found.");
    }
}
