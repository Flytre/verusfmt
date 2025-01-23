use vstd::prelude::*;

verus! {
    pub open spec fn fibo(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else if n == 1 { 1 }
    else { fibo((n - 2) as nat) + fibo((n - 1) as nat) }
}

proof fn lemma_fibo_is_monotonic(i: nat, j: nat)
    requires i <= j,
    ensures fibo(i) <= fibo(j),
    decreases j - i
{
    if i < 2 && j < 2 {
    } else if i == j {
    } else if i == j - 1 {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
    } else {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        lemma_fibo_is_monotonic(i, (j - 2) as nat);
    };
    assert(fibo(i) <= fibo(j));
}
fn main() {}

}

// cargo run microbenchmarks/proofPlumberExamples/wp_ifelse_fibo.rs --visitors RangeBoundsVisitor,QuantifierVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed
// finite proof is passes 
// (order does matter)