use std::process;

fn main() {
    println!("Process ID :: {}", get_process_id());
}

fn get_process_id() -> u32 {
    process::id()
}

#[cfg(test)]
mod tests {
    use super::get_process_id;

    #[test]
    fn test_if_process_id_is_nonzero() {
        assert_ne!(get_process_id(), 0, "There's error in code");
    }
}
