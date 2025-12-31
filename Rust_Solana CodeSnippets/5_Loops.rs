fn main() {
    // for loop
    for i in 1..=5 {
        println!("for loop: {}", i);
    }
    
    println!();
    
    // while loop
    let mut count = 0;
    while count < 3 {
        println!("while loop: {}", count);
        count += 1;
    }
    
    println!();
    
    // loop with break
    let mut num = 0;
    loop {
        if num >= 3 {
            break;
        }
        println!("loop: {}", num);
        num += 1;
    }
    
    println!();
    
    // for loop with range
    for letter in "Rust".chars() {
        println!("Letter: {}", letter);
    }
}
