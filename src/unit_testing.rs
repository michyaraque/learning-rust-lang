fn main() {}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("You cant divide by 0");
    }
    a / b
}

fn only_numbers(code: &str) -> bool {
    code.chars().all(char::is_numeric)
}

#[test]
fn check_code_only_numbers() {
    let code = "493829303";
    let result = only_numbers(code);
    assert!(result)
}

#[test]
fn check_code_only_letters() {
    let code = "49382A9303E";
    let result = only_numbers(code);
    assert!(!result, "The value contains a letter, the value was {}", code)
}

#[test]
#[ignore]
fn sum_check() {
    assert_eq!(sum(2, 2), 4)
}

#[test]
#[should_panic(expected = "You cant divide by 0")]
fn divide_by_zero_check() {
    divide(100, 0);
}
