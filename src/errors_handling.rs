use std::{fs::File, io::ErrorKind};

fn main() {

    // Not recoverables
    // Example: panic!

    // Recoverables: example:> open a file where the path is incorrect
    // Result<T, E>

    let file= File::open("./../hello.txt");
    match file {
        Ok(available_file) => read_file(available_file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found"),
            _ => println!("Unhandled exception"),
        },
    }

    // Another way with unwrap and directly panic
    //let file2 = File::open("path/new/distinct").unwrap();

}

fn read_file(file: File) -> File {
    file
}