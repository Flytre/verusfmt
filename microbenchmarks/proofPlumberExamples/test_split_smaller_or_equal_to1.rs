use vstd::prelude::*;

verus! {
    spec fn smaller_than_10(x: int) -> bool
    {
      x < 10
    }
    
    // proof fn test_split_smaller_or_equal_to()
    // {
    //   assert forall |x:int| x <= 10 implies smaller_than_10(x) by {}
    // }

    proof fn test_split_smaller_or_equal_to()
    {
      assert(forall |x:int| 0 <= x <= 10 ==> smaller_than_10(x));
    }



fn main() {}

}

// cargo run microbenchmarks/proofPlumberExamples/test_split_smaller_or_equal_to1.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor --bound 5 --print-failed
// proof fails -- proof is wrong

// proof passes! -- with small modifications