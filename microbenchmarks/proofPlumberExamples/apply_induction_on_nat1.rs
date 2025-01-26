use vstd::prelude::*;

verus! {

spec fn sum(n: nat) -> nat
{
    n * (n + 1) / 2
}

spec fn triangle(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}

proof fn sum_equal(n: nat, m: nat) by(nonlinear_arith)
    ensures sum(n) == triangle(n),
    decreases n,
{}

fn main() {}

}

// cargo run microbenchmarks/proofPlumberExamples/apply_induction_on_nat1.rs --visitors RangeBoundsVisitor,RecursionVisitor --bound 5 --print-failed