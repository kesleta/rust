//@compile-flags: -Zmiri-retag-fields
// Checks that the test does not run forever (which relies on a fast path).

#![allow(drop_copy)]

fn main() {
    let array = [(); usize::MAX];
    drop(array); // Pass the array to a function, retagging its fields
}
