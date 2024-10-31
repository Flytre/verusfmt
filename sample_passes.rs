#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
#[allow(unused_imports)]
use prelude::*;

verus! {

spec fn divides(factor: nat, candidate: nat) -> bool
    recommends 1 <= factor
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}


spec fn is_prime_F3() -> bool {
    &&& 1 < 3
    &&& forall|factor: nat| 1 < factor < 3 ==> !divides(factor, 3)
}



// spec fn divides_2_4() -> bool
//     recommends 1 <= 2
// {
//     4 % 2 == 0
// }

// spec fn divides_3_4() -> bool
//     recommends 1 <= 3
// {
//     4 % 3 == 0
// }


spec fn is_prime_F4() -> bool {
    &&& 1 < 4
    &&& !divides(2, 4)  
    &&& !divides(3, 4)
    // &&& forall|factor: nat| 1 < factor < 4 ==> !divides(factor, 4)
    // &&& divides(2,4) == (4nat % 2nat == 0)
    // &&& divides(3,4) == (4nat % 3nat == 0)
}


spec fn is_prime_F5() -> bool {
    &&& 1 < 5
    &&& forall|factor: nat| 1 < factor < 5 ==> !divides(factor, 5)
    // &&& divides(2,4) == (4nat % 2nat == 0)
    // &&& divides(3,4) == (4nat % 3nat == 0)
}


spec fn is_prime_F8() -> bool {
    &&& 1 < 8
    &&& forall|factor: nat| 1 < factor < 7 ==> !divides(factor, 8)
    &&& !divides(2, 8)  
    &&& !divides(3, 8)
    &&& !divides(4, 8)
    &&& !divides(5, 8)
    &&& !divides(6, 8)
    &&& !divides(7, 8)
}


fn main() {
    // assert(is_prime(3));

    // assert(is_prime_F5());
    assert(!is_prime_F4()); // still fails (expexted)
    // assert(forall|factor: nat| 1 < factor < 10 ==> !divides(factor, 10));
    // assert(false);
    // assert(!is_prime_F8()); 
   
}

} // verus!
