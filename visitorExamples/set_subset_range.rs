#[allow(unused_imports)]
use vstd::prelude::*;
use vstd::{map::*, prelude::*, seq::*, set::*, set_lib::*};
use std::collections::HashSet;

verus! {



    proof fn some_assertions_about_sets(a: Set<int> ,b: Set<int>) {
        
        assert(!a.subset_of(b));
        


    }


    fn main()
    {
        let mut a = HashSet::new();
        a.insert(1);

        // let b: Set<int> = set![1, 2, 4, 6, 7];
        // let a: Set<int>  = set![4, 5, 6];
    }

}