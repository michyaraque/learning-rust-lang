fn main() {
    let mut counter = 0;

    // Loop statement
    let _value = loop {
        if counter == 10 {
            // Loops can return values
            break counter;
        }
        counter += 1;
    };

    // While statement
    while counter > 0 {
        //println!("counter: {}", counter);
        counter -= 1;
    }

    // For statement

    let arr = [0, 1, 2, 3, 4, 5];

    // arr could be replaced by double dot arr creation like python 0..4
    // rev() -> reverse
    for element in arr.iter() {
        println!("{}", element);
    }
    
    //if-let
    let age: Option<i32> = Some(90);
    if let Some(value) = age {
        println!("Age: {}", value)
    }

    // while-let
    let mut unread_messages = Some(5);

    //verbosity way
    /* loop {
        match unread_messages {
            Some(value) => {
                if value > 0 {
                    println!("You have unreaded messages");
                    unread_messages = Some(value-1);
                } else {
                    println!("You dont have messages");
                    unread_messages = None;
                }
            },
            None => { break; }
        }
    } */

    //clearly way
    while let Some(value) = unread_messages {
        if value > 0 {
            println!("You have unreaded messages");
            unread_messages = Some(value-1);
        } else {
            println!("You dont have messages");
            unread_messages = None;
        }
    }
    
}
