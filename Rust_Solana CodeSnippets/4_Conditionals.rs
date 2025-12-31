fn main() {
    let age = 18;
    
    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a minor");
    }
    
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    };
    
    println!("Score: {}, Grade: {}", score, grade);
    
    let num = 7;
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}
