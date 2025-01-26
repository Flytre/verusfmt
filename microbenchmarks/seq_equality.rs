#[allow(unused_imports)]
use vstd::prelude::*;
// use vstd::{map::*, prelude::*, seq::*, set::*};

verus! {


    proof fn add_test() {
        let s1: Seq<int> = seq![0, 10, 20, 30, 40];
        let s2: Seq<int> = seq![0, 10] + seq![20] + seq![30, 40];
        //  assert(seqA =~= seqB);
         assert(s1 == s2);

     }

     proof fn test_eq_fail() {
        let s1: Seq<int> = seq![0, 10, 20, 30, 40];
        let s2: Seq<int> = seq![0, 10] + seq![20] + seq![30, 40];
        let s3: Seq<int> = Seq::new(4, |i: int| 10 * i);
        // assert(s3[2] == 20);
        assert(s1 == s2); // FAILS, even though it's true
        assert(s2 == s3); // FAILS, even is not true
    }



    fn main()
    {
        
    }

}