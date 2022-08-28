fn main() {
     // Box<T>
     let x = 2;
     let y = Box::new(x);

     println!("y = {}", y);

     // Linked lists
     // example: (value, node1) -> (value2, node2) -> (value3, null)

     enum List {
        Node(i32, Box<List>),
        None
     }

     use List::*;

     let node3 = Node(10, Box::new(None));
     let node2 = Node(6, Box::new(node3));
     let node1 = Node(90, Box::new(node2));
}