use std::fmt;

struct User {
    name: String,
    age: i32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} ({})", self.name, self.age)
    }
}

fn main() {
    let user = User {
        name: "Michy".to_string(),
        age: 25,
    };

    println!("{}", user);
}