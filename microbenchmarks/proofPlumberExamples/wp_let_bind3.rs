use vstd::prelude::*;

verus! {
    fn foo(b: u32, c: u32)
    {
        let a: u32 = b + c;
        assert(a > 10 && a < 100);
    }

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/wp_let_bind3.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed

// proof is "incorrect" so SMarTPeek Fails! 
