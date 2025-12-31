use std::fs::File;
use std::io::Read;

fn main() {
    // Option<T>
    let some_value: Option<i32> = Some(5);
    let none_value: Option<i32> = None;
    
    match some_value {
        Some(val) => println!("Found value: {}", val),
        None => println!("No value found"),
    }
    
    match none_value {
        Some(val) => println!("Found value: {}", val),
        None => println!("No value found"),
    }
    
    println!();
    
    // Using if let
    if let Some(value) = some_value {
        println!("Using if let: {}", value);
    }
    
    println!();
    
    // Result<T, E>
    let parse_result: Result<i32, _> = "42".parse();
    match parse_result {
        Ok(num) => println!("Parsed successfully: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    let bad_parse: Result<i32, _> = "hello".parse();
    match bad_parse {
        Ok(num) => println!("Parsed successfully: {}", num),
        Err(_) => println!("Could not parse 'hello' as integer"),
    }
    
    println!();
    
    // Using unwrap_or with Option
    let unknown: Option<String> = None;
    let name = unknown.unwrap_or("Unknown".to_string());
    println!("Name: {}", name);
}
