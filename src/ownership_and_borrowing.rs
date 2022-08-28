fn main() {
    //// In Rust every data contains only 1 owner

    /*
     * Stack memory [explanation]
     *
     * - Implement as stack (pile structure)
     * - Has a fixed size
     * - It is fast to read|write|delete, only need to move the pointer
     * - Released at the end of the scope
     */

    let mut age = 25;
    add_age(&mut age);
    println!("Age: {}", age);

    /*
     * Heap memory [explanation]
     *
     * - Its flexible
     * - It costs more to assign and read data due to his flexibility
     * - It is released when there are no more owners
     */
    let mut name = String::from("Michael");
    send_name(&mut name);
    println!("{}", name);
}

fn add_age(age: &mut i32) {
    {
        *age += 1;
    }
}

fn send_name(name: &mut String) {
    name.push_str("-20220827");
}
