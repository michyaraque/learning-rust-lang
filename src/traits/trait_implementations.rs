fn main() {
    let age: Option<i32> = Some(15);
    println!("You have {}", age.is_of_legal_age());
}

trait CarLicense {
    fn is_of_legal_age(&self) -> bool;
}

impl CarLicense for Option<i32> {
    fn is_of_legal_age(&self) -> bool {
        match self {
            Some(age) => age > &18,
            _ => false,
        }
    }
}
