use vstd::prelude::*;

verus! {
    fn foo()
    {
        let a: u32 = 1;
        assert(true);
        assert(a > 10 && a < 100);
    }

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/wp_assertion_step.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed

// proof is "incorrect" so SMarTPeek Fails! 
