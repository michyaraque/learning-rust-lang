use std::rc::Rc;
fn main() {
    /* Reference Counted Smart Pointer: allows a value to have many owners
     * We use Rc when we want to allocate data in the heap to be accessed in multiple parts of the code.
     * in multiple parts of the code, and we can't determine at compile time the // last one that will access this data.
     * compile time the last one that will access this data. If we knew in advance
     * who the last one would be, we could make that last one the owner,
     * but we don't know. Then Rc keeps a reference counter with
     * all owners, and when there are no more owners left, it can clean up the data.
     */

    enum List {
        Node(i32, Rc<List>),
        None,
    }

    use List::*;

    // node1 ->
    //          node2 -> value3 -> none
    // node0 ->

    let node3 = Node(10, Rc::new(None));
    let node2 = Node(6, Rc::new(node3));

    let node2_rc = Rc::new(node2);
    
    println!("References: {}", Rc::strong_count(&node2_rc));
    let node1 = Node(90, Rc::clone(&node2_rc));
    println!("References: {}", Rc::strong_count(&node2_rc));
    let node0 = Node(5, Rc::clone(&node2_rc));
    println!("References: {}", Rc::strong_count(&node2_rc));
}
