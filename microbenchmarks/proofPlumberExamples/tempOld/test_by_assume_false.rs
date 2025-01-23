use vstd::prelude::*;

verus! {

    // ------- test_by_assume_false1 ---------

    proof fn f(x: int) { 
        assert(x == 3); 
    }

    proof fn f(x: int) { 
        assert(x == 3) by {
            assume(false);
        }; 
    }

    // ------- test_by_assume_false2 ---------



    spec fn pow2(e: nat) -> nat 
        decreases(e),
    {
        if e == 0 { 1 } else { 2 * pow2((e - 1) as nat)}
    }

    proof fn lemma_pow2_unfold3(e: nat) 
        requires e > 3,
        ensures pow2(e) == pow2((e-3) as nat) * 8,
    {
        assert(pow2(e) == pow2((e - 3) as nat) * 8);
    }



    proof fn lemma_pow2_unfold3(e: nat) 
        requires e > 3,
        ensures pow2(e) == pow2((e-3) as nat) * 8,
    {
        assert(pow2(e) == pow2((e - 3) as nat) * 8) by {
            assume(false);
        };
    }


    // ------- test_by_assume_false3 ---------

    // proof fn f(a: u64, b: u64) { 
    //     assert((a & (a | b)) == a);
    // }


    proof fn f(a: u64, b: u64) { 
        assert((a & (a | b)) == a) by {
            assume(false);
        };
    }

    //I dont understand why assume false is helpful


    fn main(){}

}