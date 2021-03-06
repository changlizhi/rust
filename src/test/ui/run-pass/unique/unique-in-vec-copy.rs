// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![feature(box_syntax)]

pub fn main() {
    let mut a: Vec<Box<_>> = vec![box 10];
    let b = a.clone();

    assert_eq!(*a[0], 10);
    assert_eq!(*b[0], 10);

    // This should only modify the value in a, not b
    *a[0] = 20;

    assert_eq!(*a[0], 20);
    assert_eq!(*b[0], 10);
}
