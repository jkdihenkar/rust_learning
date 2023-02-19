use process_lib;

#[test]

fn integration_test1() {
    assert_ne!(process_lib::get_process_id(), 0, "Error
        in code");
}


#[test]
#[ignore]
fn process_test1() {
    assert!(true);
}

