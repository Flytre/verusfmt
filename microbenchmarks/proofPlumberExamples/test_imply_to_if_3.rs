use vstd::prelude::*;

verus! {

    fn test_if(a: u32, b: u32) {
        assert(a == 0xffffffff ==> a & b == b);
    }


fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/test_imply_to_if_3.rs --visitors RangeBoundsVisitor,RecursionVisitor,RecursiveDatatypeVisitor,ModularFlattenerVisitor,FunctionInlineVisitor --bound 5 --print-failed
// Proof succeeds 

// yes in finite range b/c bounds get set to:
    // requires
    //     a >= 0,
    //     a <= 5,
    //     b >= 0,
    //     b <= 5,
// in fact any value of b is valid for the function to hold the assertion.