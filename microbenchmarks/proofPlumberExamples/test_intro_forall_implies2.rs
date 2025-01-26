

use vstd::prelude::*;

verus! {

    #[verifier::opaque]
    spec fn f1(x: int, y: int) -> bool
    {
    x * x * x  ==  y * y * y
    }

    proof fn test_intro_forall_implies2() {
        assert(forall|i: int, j: int| i == j ==> f1(i, j) && f1(i, j));
      
      }

    fn main(){
     
    }
}

// cargo run microbenchmarks/proofPlumberExamples/test_intro_forall_implies2.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor,RevealVisitor --bound 5 --print-failed
// proof passes! 

// SAME AS test_intro_forall_2