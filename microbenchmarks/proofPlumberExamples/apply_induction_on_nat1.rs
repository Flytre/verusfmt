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

proof fn sum_equal(n: nat, m: nat)
    ensures sum(n) == triangle(n),
    decreases n,
{}

fn main() {}


}