use std::thread;
use std::time::Duration;

fn main() {
    // handler
    let name = String::from("Michael");
    let name_clone = name.clone(); // Its cost 

    let join_handle = thread::spawn(move || {
        println!("Thread 1 | {} is online", name);
        thread::sleep(Duration::from_millis(3000));
    });

    join_handle.join().unwrap();
    
    println!("Thread principal |: What do you need {}?", name_clone);

}