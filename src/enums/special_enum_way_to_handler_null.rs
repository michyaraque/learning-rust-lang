#[allow(dead_code)]

struct User {
    name: String,
    age: Option<i32>,
}

impl User {
    fn get_age(&self) -> i32 {
        if self.age.is_none() {
            return -1;
        }
        self.age.unwrap()/* .unwrap_or_default() */
    }
}

fn main() {
    let new_user = User {
        name: String::from("Michy"),
        age: Some(12)
    };

    
    let age = new_user.get_age();

    // return unwrap or default by type directly from the implementation (impl get_age)
    println!("The age of the user is {}", age)

    // match logic way
    /* match age {
        // Underscore implicit means do all of the other values on the Option Enums
        Some(age) => println!("The age of the user is {}", age),
        _ => (),
    } */
}
