fn main() {
    // Ownership basics
    let s1 = String::from("Hello");
    let s2 = s1; // s1's value is moved to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This would error - s1 no longer owns the data
    
    println!();
    
    // Borrowing (References)
    let s3 = String::from("World");
    let len = calculate_length(&s3); // Borrow s3
    println!("String: {}, Length: {}", s3, len);
    
    println!();
    
    // Mutable borrowing
    let mut s4 = String::from("Rust");
    change_string(&mut s4); // Mutable borrow
    println!("Changed string: {}", s4);
    
    println!();
    
    // Copy trait (primitives)
    let x = 5;
    let y = x; // Copy happens automatically for integers
    println!("x: {}, y: {}", x, y); // Both are valid
    
    println!();
    
    // Slice
    let s5 = String::from("Blockchain");
    let slice = &s5[0..5];
    println!("Slice: {}", slice);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" Programming");
}
