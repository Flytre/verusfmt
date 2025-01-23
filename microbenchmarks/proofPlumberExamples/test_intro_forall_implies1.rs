

use vstd::prelude::*;

verus! {

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

    fn main(){}

}    //verifies already  



