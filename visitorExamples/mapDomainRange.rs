#[allow(unused_imports)]
use vstd::prelude::*;
use vstd::{map::*, prelude::*, seq::*, set::*};

verus! {


    proof fn add_test_map(m1: Map<int,int>, i:int) -> (m2: Map<int,int>)
        requires 
            !m1.dom().contains(i),
        ensures 
            m2.dom().len() > m1.dom().len()
    {
        let m2 = m1.insert(i,42int);
        m2
     }

    //  pub proof fn tracked_insert(tracked &mut self, key: K, tracked value: V)
//      ensures
//          *self == Map::insert(*old(self), key, value),
//  {
//      unimplemented!();
//  }

    fn main()
    {
        
    }

}