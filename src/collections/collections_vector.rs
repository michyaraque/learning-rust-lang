fn main() {
    // Vector [all values had to have the same type]
    let _vector: Vec<i32> = Vec::new();
    let mut vector = vec![1, 2, 3];

    vector.push(9);
    vector.push(24);

    let third_value = vector.get(3);
    if third_value.is_some() {
        println!("Value: {}", third_value.unwrap());
    }

    println!("\nSimple vector iteration");

    for iteration in &vector {
        println!("Value: {}", iteration);
    }

    // we can modify the vector through reference + mut
    println!("\nModify through reference");

    for iteration in &mut vector {
        *iteration += 10;
    }

    for iteration in &vector {
        println!("Value: {}", iteration);
    }

    println!("\nVector with multiple types");
    enum Message {
        TEXT(String),
        ERROR(i32)
    }

    let messages = vec![Message::TEXT("Hello".to_string()), Message::ERROR(404)];
    for message in &messages {
        match message {
            Message::TEXT(text) => println!("Message: {}", text),
            Message::ERROR(code) => println!("Error: {}", code)
        }
        
    }

}

