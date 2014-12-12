// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that unboxed closures in contexts with free type parameters
// monomorphize correctly (issue #16791)

#![feature(unboxed_closures)]

fn main(){
    fn bar<'a, T:'a> (t: T) -> Box<FnOnce<(),T> + 'a> {
        box move |:| t
    }

    let f = bar(42u);
    assert_eq!(f.call_once(()), 42);

    let f = bar("forty-two");
    assert_eq!(f.call_once(()), "forty-two");

    let x = 42u;
    let f = bar(&x);
    assert_eq!(f.call_once(()), &x);

    #[deriving(Show, PartialEq)]
    struct Foo(uint, &'static str);

    impl Copy for Foo {}

    let x = Foo(42, "forty-two");
    let f = bar(x);
    assert_eq!(f.call_once(()), x);
}