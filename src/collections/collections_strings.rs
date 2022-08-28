fn main() {
    // String created dynamically on the Heap memory
    let parsed_string: String = "Michael".to_string();
    // Hardcoded value on stack memory
    let _literall_name = "Michael";

    // Convert String to string slice
    let _new_name: &str = &parsed_string[..parsed_string.len()];
}
