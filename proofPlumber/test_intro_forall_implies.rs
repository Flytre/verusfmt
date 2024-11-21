

use vstd::prelude::*;

verus! {

    // ------- test_intro_forall_implies1 ---------

    #[verifier::opaque]
    spec fn twice(x: int) -> int
    {
      x * 2
    }
    
    proof fn test_intro_forall_implies1() {
      assert(forall|x:int, y:int| x <= y ==> twice(x) <= twice(y)) by {
        reveal(twice);
      }
    }

    
    // proof fn test_intro_forall_implies1() {
    //   assert forall|x: int, y: int| x <= y implies twice(x) <= twice(y) by {
    //         reveal(twice);
    //     }
    // }

    // ------- test_intro_forall_implies2 ---------

    #[verifier::opaque]
    spec fn f1(x: int, y: int) -> bool
    {
        x * x * x  ==  y * y * y
    }

    proof fn test_intro_forall_implies2() {
        assert(forall|i: int, j: int| i == j ==> f1(i, j) && f1(i, j));
    }

    // proof fn test_intro_forall_implies2() {
    //  assert forall|i: int, j: int| i == j implies f1(i, j) && f1(i, j) by {};

    // }



    fn main(){}

}