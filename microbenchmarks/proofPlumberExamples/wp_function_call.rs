use vstd::prelude::*;

verus! {
    fn octuple(x1: i8) -> (x8: i8)
    requires
        -16 <= x1,
        x1  < 16,
    ensures
        x8 == 8 * x1,
    {
        let x2 = x1 + x1;
        let x4 = x2 + x2;
        x4 + x4
    }
    
    fn use_octuple() {
        let two = 2;
        let num = octuple(two);
        assert(num == 32);
    }
fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/wp_function_call.rs --visitors RangeBoundsVisitor,RecursionVisitor,RecursiveDatatypeVisitor,ModularFlattenerVisitor,FunctionInlineVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
