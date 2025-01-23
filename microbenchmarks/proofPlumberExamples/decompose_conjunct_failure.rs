use vstd::prelude::*;

verus! {
    fn foo()
    {
        let a:u32 = 1;
        assert(a > 10 && a < 100);
    }

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/decompose_conjunct_failure.rs --visitors RangeBoundsVisitor,RecursionVisitor,RecursiveDatatypeVisitor,ModularFlattenerVisitor,FunctionInlineVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
