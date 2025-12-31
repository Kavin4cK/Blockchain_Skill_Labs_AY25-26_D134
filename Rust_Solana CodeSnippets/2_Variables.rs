fn main() {
    // Immutable variable
    let x = 5;
    println!("x = {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y after change = {}", y);
    
    // Different data types
    let int_num: i32 = 42;
    let float_num: f64 = 3.14;
    let is_active: bool = true;
    let character: char = 'R';
    
    println!("int: {}, float: {}, bool: {}, char: {}", int_num, float_num, is_active, character);
}
