use vstd::prelude::*;

verus! {

    proof fn lemma_mul_inequality(x: int, y: int, z: int) 
    by(nonlinear_arith)
    ensures  x <= y && z > 0 ==> x * z <= y * z,
{
}


fn main() {}

}
  //verifies already  