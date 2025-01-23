use vstd::prelude::*;

verus! {

proof fn lemma_mul_inequality(x: int, y: int, z: int) by(nonlinear_arith)
    requires x <= y && z > 0
    ensures  x * z <= y * z    
{}

proof fn lemma_mul_strict_upper_bound(x: int, xbound: int, y: int, ybound: int)
    requires x < xbound && y < ybound && 0 <= x && 0 <= y
    ensures x * y <= (xbound - 1) * (ybound - 1)
{
    lemma_mul_inequality(x, xbound - 1, y);
    lemma_mul_inequality(y, ybound-1, xbound-1);
}

fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/intro_requires_mul_ineq.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
