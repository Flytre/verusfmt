use vstd::*;
use vstd::prelude::*;

verus! {



spec fn triangle(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}
// 

fn main() {
    // assert(triangle(0) == 0);
    // assert(triangle(1) == 1);
    // assert(triangle(2) == 3);

    assert(triangle(5) == 15);
}


}