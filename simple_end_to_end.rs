#[allow(unused_imports)]
use prelude::*;
#[allow(unused_imports)]
use seq::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use vstd::*;

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


fn main() {

    assert(is_prime(11));
    
}

} // verus!
// vargo run -p rust_verify --release rust_verify/example/summer_school/chapter-2-1.rs
