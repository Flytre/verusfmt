#[allow(unused_imports)]
use vstd::prelude::*;
use vstd::{map::*, prelude::*, seq::*, set::*, set_lib::*};

verus! {



    proof fn some_assertions_about_sets() {
        let b: Set<int> = set![1, 2, 4, 6, 7];
        let a: Set<int>  = set![4, 5, 6];
        assert(!a.subset_of(b));
        


    }


    fn main()
    {
        
    }

}