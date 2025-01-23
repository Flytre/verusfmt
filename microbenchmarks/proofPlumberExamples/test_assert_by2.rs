use vstd::prelude::*;

verus! {

spec fn pow2(e: nat) -> nat 
    decreases(e),
{
    if e == 0 { 1 } else { 2 * pow2((e - 1) as nat)}
}

proof fn lemma_pow2_unfold3(e: nat) 
    requires e > 3,
    ensures pow2(e) == pow2((e-3) as nat) * 8,
{
    assert(pow2(e) == pow2((e - 3) as nat) * 8);
}

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/test_assert_by2.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor --bound 5 --print-failed
// finite proof is passes



//SAME as "test_by_assume_false2"
