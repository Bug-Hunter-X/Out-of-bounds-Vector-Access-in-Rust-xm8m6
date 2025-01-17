fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let value = vec.get(1);
    match value {
        Some(val) => println!("Value at index 1: {}", val),
        None => println!("Index out of bounds"),
    }
} 