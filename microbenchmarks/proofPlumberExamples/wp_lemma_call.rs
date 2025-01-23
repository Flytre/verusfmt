use vstd::prelude::*;

verus! {
proof fn commutative(a: int, b: int)
    ensures a*b == b*a,
{
    assume(false);
}

proof fn foo()
{
    let v1:int = 100;
    let v2:int = 200;
    commutative(v1, v2);
    assert(false);
}

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/wp_lemma_call.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed
// proof passes! -- with small modifications

// --> proof is too weak to prove assert(false)