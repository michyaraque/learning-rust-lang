/* const SPACES: i32 = 3; */

fn main() {

    // Shadowing --------------------------------
    /* println!("Spaces as constant {}", SPACES);

    let tabs = "   ";
    println!("User wants '{}' tabs", tabs);
    let tabs = tabs.len();
    println!("Value of tabs is {}", tabs); */

    // Types ------------------------------------
    /* let array = [1, 2, 3, 4, 5];

    let name: String = String::from("Michael");
    let mut last_name: String = String::new();
    last_name = String::from("Araque"); */
    show_welcome();
    let age = show_your_age(&25);
    println!("Actual age {}", age);
    
    let greetings = show_welcome_with_name("Michy");
    println!("Greetings {}", greetings);

}

fn show_welcome_with_name(name: &str) -> &str {
    return name;
}

fn show_welcome() {
    println!("Welcome");
}

// Dont need semicolon if you want to not to use return statement
fn show_your_age(age: &i32) -> i32 {
    *age + 5
}