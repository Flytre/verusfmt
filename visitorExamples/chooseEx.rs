   #![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {
    spec fn less_than(x: int, y: int) -> bool {
        x < y
    }

    proof fn test_choose_succeeds2() {
        // assert(less_than(3, 7));  // promote i = 3, i = 7 as a witness
        // assert(less_than(0, 0) || less_than(0, 1));
        let (x, y) = choose|i: int, j: int| less_than(i, j);
        assert(x < y);
    }


    fn main()

    {

    }

}