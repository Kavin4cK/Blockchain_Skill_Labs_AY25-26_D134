struct Person {
    name: String,
    age: u32,
    email: String,
}

struct Point {
    x: i32,
    y: i32,
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Person {
        Person { name, age, email }
    }
    
    fn introduce(&self) {
        println!("Hi, I'm {} and I'm {} years old", self.name, self.age);
    }
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (((self.x.pow(2) + self.y.pow(2)) as f64).sqrt())
    }
}

fn main() {
    // Create struct instance
    let person = Person::new("Alice".to_string(), 25, "alice@example.com".to_string());
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Email: {}", person.email);
    
    person.introduce();
    
    println!();
    
    // Point struct
    let point = Point { x: 3, y: 4 };
    println!("Point: ({}, {})", point.x, point.y);
    println!("Distance from origin: {}", point.distance_from_origin());
}
