use adder::add_two;
mod common;
mod integration;
#[test]
fn it_adds_two() {
    common::setup();
    integration::test_for_integration();
    common::gen::test_integration2();
    let result = add_two(2);
    assert_eq!(result, 4);
}