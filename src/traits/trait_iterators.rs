fn main() {
    let numbers = [1, 2, 3];

    for number in numbers.iter() {
        println!("{}", number + 2);
    }

    let mut counter = Counter::new();
    counter.next();
    counter.next();
    let iteration = counter.next();

    match iteration {
        Some(count) => println!("{}", count),
        None => println!("End of count", ),
    }
}
    

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Iterator trait
// Handle number variations
impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}