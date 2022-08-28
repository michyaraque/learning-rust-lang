struct User {
    name: String,
    email: String,
    year: i32,
    is_active: bool,
}

impl User {
    fn age(&self) -> i32 {
        let actual_year = 2022;
        actual_year - self.year
    }
}

fn main() {
    // shorthand struct init
    let new_user = new_user(String::from("Michy"), String::from("tester@example.com"));

    println!("New user born year {}", new_user.year);

    let mut user = User {
        name: String::from("Michael"),
        email: String::from("test@test.com"),
        ..new_user
    };

    user.is_active = true;

    println!(
        "User data | name: {}, email: {}, ¿is_active?: {}, age: {}",
        user.name, user.email, user.is_active, user.age()
    );

    // tuple structs
    // struct Point(i32, i32, i32);
    // let pointA = Point(12, 50, 90);
}

fn new_user(name: String, email: String) -> User {
    return User {
        name,
        email,
        year: 1997,
        is_active: false,
    };
}
