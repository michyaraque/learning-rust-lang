#[allow(dead_code)]

/* enum Option<T> {
    Some(T),
    None,
} */
// Null concept

fn main() {
    // In case of null exception
    //let name: Option<String> = None;
    
    let name: Option<String> = Some(String::from("Michy"));

    match name {
        None => println!("Name doesn't not found"),
        Some(name) => println!("{}", name)
    }
    
}