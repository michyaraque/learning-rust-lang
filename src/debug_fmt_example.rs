#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}
// Its large to use
/* impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "User {}, age: {}", self.name, self.age)
    }
} */
fn main() {
    let user = User {
        name: "Michy".to_string(),
        age: 25,
    };

    println!("{:?}", user);
}