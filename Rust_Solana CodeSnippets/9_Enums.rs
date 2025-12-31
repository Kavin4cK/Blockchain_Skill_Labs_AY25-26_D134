enum Color {
    Red,
    Green,
    Blue,
}

enum Status {
    Active,
    Inactive,
    Pending,
}

enum Result_Type {
    Success(String),
    Error(String),
}

fn check_status(status: Status) {
    match status {
        Status::Active => println!("System is active"),
        Status::Inactive => println!("System is inactive"),
        Status::Pending => println!("System is pending"),
    }
}

fn main() {
    let color = Color::Blue;
    match color {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
    }
    
    println!();
    
    check_status(Status::Active);
    check_status(Status::Pending);
    
    println!();
    
    let result = Result_Type::Success("Operation completed".to_string());
    match result {
        Result_Type::Success(msg) => println!("Success: {}", msg),
        Result_Type::Error(msg) => println!("Error: {}", msg),
    }
    
    let error = Result_Type::Error("Something went wrong".to_string());
    match error {
        Result_Type::Success(msg) => println!("Success: {}", msg),
        Result_Type::Error(msg) => println!("Error: {}", msg),
    }
}
