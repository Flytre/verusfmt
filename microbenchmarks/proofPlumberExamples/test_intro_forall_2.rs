

use vstd::prelude::*;

verus! {

    #[verifier::opaque]
    spec fn f1(x: int, y: int) -> bool
    {
    x * x * x  ==  y * y * y
    }

    // proof fn test_intro_forall_implies() {
    //     assert(forall|i: int, j: int| i == j ==> f1(i, j) && f1(i, j));

    // }

    proof fn test_intro_forall_implies() {
        // reveal(f1);
        //  assert forall|i: int, j: int| i == j ==> f1(i, j) && f1(i, j) by {};
        assert(forall|i: int, j: int| i == j ==> f1(i, j) && f1(i, j));
        // 0 == 1 ==> 

    }

    fn main(){
      
       
    }

}