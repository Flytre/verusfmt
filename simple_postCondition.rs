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

    spec fn divides(factor: nat, candidate: nat) -> bool
    recommends
        1 <= factor,
{
    candidate % factor == 0
}


spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}

proof fn test()
    ensures is_prime(7)
{
    assert(is_prime(7));
    
    // let candidate = 7;
    // assert forall|factor: nat| 1 < factor < candidate implies !divides(factor, candidate) by {
    //     assert(!divides(2, candidate));
    //     assert(!divides(3, candidate));
    //     assert(!divides(4, candidate));
    //     assert(!divides(5, candidate));
    //     assert(!divides(6, candidate));  
    // }
    
}



fn main() {

    assert(is_prime(11));
    proof{
        let candidate = 7;
   
        assert(!divides(2, candidate));
    }
    
}

} // verus!
