use vstd::prelude::*;

verus! {

proof fn my_proof_fun(x: int, y: int)
    requires
        x < 100,
        y < 100,
    ensures
        x + y < 200,
        x + y < 100,
{
    assert(x + y < 600);
}

fn main() {}

}


// cargo run microbenchmarks/proofPlumberExamples/intro_failing_ensures_easy.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor --bound 51 --print-failed

// (NOTE - depends on the bound! )
// w/sufficiently large bound, the proof fails (as it should)