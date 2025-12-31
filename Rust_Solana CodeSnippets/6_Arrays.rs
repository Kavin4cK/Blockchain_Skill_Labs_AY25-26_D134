fn main() {
    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("First element: {}", arr[0]);
    println!("Array length: {}", arr.len());
    
    println!();
    
    // Vector
    let mut vec = vec![10, 20, 30];
    println!("Vector: {:?}", vec);
    
    vec.push(40);
    println!("After push: {:?}", vec);
    
    vec.pop();
    println!("After pop: {:?}", vec);
    
    println!();
    
    // Iterate over array
    for item in arr.iter() {
        println!("Item: {}", item);
    }
    
    println!();
    
    // Iterate over vector
    for item in &vec {
        println!("Vector item: {}", item);
    }
}
