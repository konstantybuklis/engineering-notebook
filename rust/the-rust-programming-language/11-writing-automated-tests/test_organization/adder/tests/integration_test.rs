// Integration tests only work for the lib crate, since only functions can be imported from it,
// but not from the bin crate

mod common;

use adder::add_two;

#[test]
fn id_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
