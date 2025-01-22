

use vstd::prelude::*;

verus! {

    #[verifier::opaque]
    spec fn twice(x: int) -> int
    {
      x * 2
    }
    
    proof fn test_intro_forall() {
      assert(forall|x: int, y: int| twice(x) + twice(y) == x*2 + y*2) by {
        reveal(twice);
      }
    }

    //verifies already  

    // proof fn test_intro_forall() {
    //     assert forall|x: int, y: int| twice(x) + twice(y) == x * 2 + y * 2 by {
    //           reveal(twice);
    //       }
    //   }



    fn main(){}

}