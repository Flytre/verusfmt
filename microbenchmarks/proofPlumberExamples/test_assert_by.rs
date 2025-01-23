use vstd::prelude::*;

verus! {

proof fn f() { 
    assert(x == 3);
}

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/test_assert_by.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 


//SAME as "test_by_assume_false1"

