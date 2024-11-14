#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
#[allow(unused_imports)]
use prelude::*;

verus! {


// assertion fails
// no impl to check
// proof passes

spec fn divides(factor: nat, candidate: nat) -> bool
    recommends 1 <= factor
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}





// spec fn is_prime_F3() -> bool {
//     &&& 1 < 3
//     &&& forall|factor: nat| 1 < factor < 3 ==> !divides(factor, 3)
// }





fn main() {

    assert(is_prime(3));


   
}

} // verus!
