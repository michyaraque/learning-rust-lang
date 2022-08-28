use std::ops::Deref;

fn example_one() {
    let x = 5;
    let y = &x;
    let z = &y;

    if x == 5 {
        println!("Works");
    }

    // can't compare `&{integer}` with `{integer}`
    /* if y == 5 {
        println!("Works");
    } */

    if *y == 5 {
        println!("Works");
    }

    // double derefentiation
    if **z == 5 {
        println!("Works");
    }
}

fn example_two() {
    let x = 5;
    let y = Box::new(x);

    if *y == 5 {
        println!("Works");
    }
}

fn example_three() { // contains an example of a custom Deref
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    if *y == 5 {
        println!("Works")
    }
}

fn main() {
    // Defer trait: allows to dereferencing (*)
    example_one();
    example_two();
    example_three();
}


