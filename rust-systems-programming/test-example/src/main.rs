use std::process;

fn main() {
    println!("Process ID :: {}", get_process_id());
}

fn get_process_id() -> u32 {
    process::id()
}

#[test]
fn test_if_process_id_is_returned() {
    assert!(get_process_id() > 0);
}
