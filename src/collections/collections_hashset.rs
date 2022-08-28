use std::collections::HashSet;
fn main() {
    //let hash:HashSet<i32> = vec![1..10];
    let mut user_ids = HashSet::new();
    user_ids.insert(234985422);
    user_ids.insert(198440902);
    user_ids.insert(598293993);

    /* for id in user_ids.iter() {
        println!("{}", id);
    } */

    let mut backup_ids = HashSet::new();
    backup_ids.insert(234985422);
    backup_ids.insert(439844090);
    backup_ids.insert(198293993);

    for id in user_ids.difference(&backup_ids) {
        println!("{}", id)
    }
}
