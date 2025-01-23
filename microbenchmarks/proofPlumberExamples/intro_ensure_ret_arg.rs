use vstd::prelude::*;

verus! {


proof fn my_proof_fun(x: int, y: int) -> (sum: int)
    requires
        x < 100,
        y < 100,
    ensures
        sum < 100,
        sum < 200,
        sum < 300,
{
    assert(x + y < 200);
    x + y
}

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/intro_ensure_ret_arg.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor --bound 51 --print-failed
// (NOTE - depends on the bound! )
// w/sufficiently large bound, the proof fails (as it should)