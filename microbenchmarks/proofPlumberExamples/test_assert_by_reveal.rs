use vstd::prelude::*;

verus! {

    pub open spec fn fibo(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else if n == 1 { 1 }
    else { fibo((n - 2) as nat) + fibo((n - 1) as nat) }
}


  #[verifier::opaque]
  spec fn opaque_fibo(n: nat) -> nat
      decreases n
  {
      if n == 0 { 0 } else if n == 1 { 1 }
      else { fibo((n - 2) as nat) + fibo((n - 1) as nat) }
  }

  proof fn test_opaque_fibo() 
  {
      assert(opaque_fibo(2) == 1);
  }

fn main() {}

}
//cargo run microbenchmarks/proofPlumberExamples/test_assert_by_reveal.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor --bound 5 --print-failed
// finite proof is passes 
// (order does matter)
// SAME as test_assert_by_reveal1