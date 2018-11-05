extern crate ch_11_03_02_01;

mod common;

#[test]
fn inte_test_01() {
    common::setup();
    assert_eq!(ch_11_03_02_01::add(2, 2), 4);
}
