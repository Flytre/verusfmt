#[allow(unused_imports)]
use vstd::prelude::*;
use vstd::{map::*, prelude::*, seq::*, set::*};

verus! {


    struct Foo {
        a: Seq<int>,
        b: Set<int>,
    }

    proof fn add_test() {
        let s1: Seq<int> = seq![0, 10, 20, 30, 40];
        let s2: Seq<int> = seq![0, 10] + seq![20] + seq![30, 40];
        //  assert(seqA =~= seqB);
         assert(s1 == s2);

     }

     proof fn test_eq_fail() {
        let s1: Seq<int> = seq![0, 10, 20, 30, 40];
        let s2: Seq<int> = seq![0, 10] + seq![20] + seq![30, 40];
        let s3: Seq<int> = Seq::new(5, |i: int| 10 * i);
        assert(s1 == s2); // FAILS, even though it's true
        assert(s1 === s3); // FAILS, even though it's true
    }


    fn main()
    {
        
    }

}