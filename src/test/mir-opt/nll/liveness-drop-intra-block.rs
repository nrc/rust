// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags:-Znll

#![allow(warnings)]

fn use_x(_: usize) -> bool { true }

fn main() {
    let mut x = 22;
    loop {
        // Key point: `x` not live on entry to this basic block.
        x = 55;
        if use_x(x) { break; }
    }
}

// END RUST SOURCE
// START rustc.node12.nll.0.mir
//    | Variables regular-live on entry to the block bb1: []
//    | Variables drop-live on entry to the block bb1: []
//    bb1: {
//        | Regular-Live variables here: []
//        | Drop-Live variables here: []
//        _1 = const 55usize;
//        | Regular-Live variables here: [_1]
//        | Drop-Live variables here: []
//        StorageLive(_3);
//        | Regular-Live variables here: [_1]
//        | Drop-Live variables here: []
//        StorageLive(_4);
//        | Regular-Live variables here: [_1]
//        | Drop-Live variables here: []
//        _4 = _1;
//        | Regular-Live variables here: [_4]
//        | Drop-Live variables here: []
//        _3 = const use_x(_4) -> bb2;
//    }
// END rustc.node12.nll.0.mir
