fn main() {
    // String creation
    let s1 = String::from("Hello");
    let s2 = " World".to_string();
    
    // String concatenation
    let greeting = format!("{}{}", s1, s2);
    println!("{}", greeting);
    
    println!();
    
    // String methods
    let text = "Rust Programming";
    println!("Original: {}", text);
    println!("Length: {}", text.len());
    println!("Uppercase: {}", text.to_uppercase());
    println!("Lowercase: {}", text.to_lowercase());
    
    println!();
    
    // String slicing
    let word = "Blockchain";
    let slice = &word[0..5];
    println!("Slice of '{}': {}", word, slice);
    
    println!();
    
    // Contains and starts_with
    let sentence = "Learning Rust is fun";
    println!("Contains 'Rust': {}", sentence.contains("Rust"));
    println!("Starts with 'Learning': {}", sentence.starts_with("Learning"));
    
    println!();
    
    // Split
    let csv = "apple,banana,cherry";
    for fruit in csv.split(',') {
        println!("Fruit: {}", fruit);
    }
}
