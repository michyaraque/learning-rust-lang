fn main() {
    let michy = Human;
    println!("{}", michy.say_hello());

    let the_cat = Cat;
    println!("{}", the_cat.say_hello());

    println!("\n{}", Cat::language());
    println!("{}", Human::language());
}

struct Human;
struct Cat;

trait Talk {
    fn say_hello(&self) -> String;
    fn language() -> String {
        String::from("No language")
    }
}

impl Talk for Human {
    fn say_hello(&self) -> String {
        "Hello guys".to_string()
    }

    fn language() -> String {
        "Spanish".to_string()
    }
}

impl Talk for Cat {
    fn say_hello(&self) -> String {
        "Miau".to_string()
    }

    fn language() -> String {
        "Cat lang".to_string()
    }
}