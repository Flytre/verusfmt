use vstd::prelude::*;

verus! {
    fn foo()
    {
        let a: u32 = 1;
        assert(a > 10 && a < 100);
    }



fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/wp_let_bind.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed

//proof fails - no finitization to be done