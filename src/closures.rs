fn main()  {

    let sum = sum;
    println!("{}", sum(5));

    // | pipe
    let multiply = |number: i32| -> i32 {
        number * 5
    };

    println!("{}", multiply(5));
}

fn sum(number: i32) -> i32 {
    number + 1
}