use vstd::prelude::*;

verus! {
    fn foo()
    {
        let mut a: u32 = 1;
        if (a > 10) {
            a = 2;
        } else if (a > 100) {
            a = 3;
        } else {
            a = 4;
        };
        assert(a > 10 && a < 100);
    }

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/wp_if_else_nested.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed

// proof is "incorrect" so SMarTPeek Fails! 
