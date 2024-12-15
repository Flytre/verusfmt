#[allow(unused_imports)]
use vstd::prelude::*;
use vstd::{map::*, prelude::*, seq::*, set::*};

verus! {


    struct Foo {
        a: Seq<int>,
        b: Set<int>,
    }

    proof fn add_test(s1: Set<int>, i:int) 
        requires 
            s1.contains(i),
        ensures 
            s1.len() > 0
    {
        assert(s1.contains(i));

     }

    fn main()
    {
        
    }

}