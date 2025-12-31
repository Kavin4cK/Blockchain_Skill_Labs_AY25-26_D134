fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    greet("Alice");
    greet("Bob");
    
    let answer = multiply(4, 7);
    println!("4 * 7 = {}", answer);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
