fn main() {
    /*
     * Lifetime
     *
     * - Lifetime of the memory reference
     * The concept of lifetime is a way of ensuring that a
     * piece of memory is available for reference.
     *
     * - Lifetime only available for references
     */

    do_something(&59, "Hello");

    let a;

    // internal scope
    {
        let b = 10;
        a = b; // now have the ownership
    } // b is going to stop its life when the scope is over

    println!("The value of a is: {}", a);

    let name: &'static str = "Michael Static";
}

// With lifetime. Rust infers automatically this lifetimes but the dev can specify it
fn do_something<'a>(param_a: &'a i32, param_b: &'a str) -> &'a i32 {
    param_a
}

// Without lifetime [static]
fn do_something_static(param_a: i32, param_b: String) -> i32 {
    50
}
