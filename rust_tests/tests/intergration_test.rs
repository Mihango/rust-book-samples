#![allow(unused)]

use rust_tests;

mod commons;

#[test]
fn it_adds_two() {
    commons::setup();
    assert_eq!(4, rust_tests::add_two(2))
}