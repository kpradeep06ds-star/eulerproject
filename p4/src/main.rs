fn is_palindrome(x: i64) -> bool {
    if x < 0 { return false; }
    let original = x;
    let mut reversed = 0;
    let mut temp = x;

    while temp > 0 {
        reversed = reversed * 10 + (temp % 10);
        temp /= 10;
    }

    original == reversed
}

fn main() {
    let mut palindromes: Vec<i64> = Vec::new();

    for i in (100000..=999999).rev() {
        if is_palindrome(i) {

            let mut found = false;
            let root = (i as f64).sqrt() as i64;
            
            // Start from the root and go up to 999
            for factor1 in (root..=999).rev() {
                if i % factor1 == 0 {
                    let factor2 = i / factor1;
                    if factor2 >= 100 && factor2 <= 999 {
                        palindromes.push(i);
                        found = true;
                        break;
                    }
                }
            }
            if found { break; } 
        }
    }
    
    if let Some(largest) = palindromes.first() {
        println!("The largest palindrome is: {}", largest);
    }
}