#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
use set::*;
#[allow(unused_imports)]
use prelude::*;
use multiset::*;

verus! {


spec fn arith_sum_int(i: int) -> int
    decreases i,
{
    if i <= 0 {
        0
    } else {
        i + arith_sum_int(i - 1)
    }
}


fn compute_arith_sum(n: u64) -> (sum: u64)
    ensures
        arith_sum_int(n as int) == sum,
{
    let mut sum: u64 = 0;
    for i in 0..n-1
        invariant
            arith_sum_int(i as int) == sum,
    {
        sum = sum + (i + 1);
    }
    sum
}


fn main() {


    
}

} // verus!
