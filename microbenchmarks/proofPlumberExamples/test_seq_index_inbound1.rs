use vstd::prelude::*;
use vstd::seq::*;
use vstd::seq_lib::*;

verus! {

    proof fn test_seq_in_bound() {
        let s1 = Seq::new(6, |i: int|
          if i == 0 { 10int }
          else if i == 1 { 20int }
          else if i == 2 { 30int }
          else if i == 3 { 45int }
          else if i == 4 { 55int }
          else { 70 }
        );
        let s2: Seq<int> = s1.filter(|x: int| x < 40);
        assert(forall|i: int| s2[i] < 40);

        // assert(forall|i: int| 0 <= i < s2.len() ==> s2[i] < 40);
    }


fn main() {}

}

//fixed since this was not parsing -- adding 'int' to constants
//cargo run microbenchmarks/proofPlumberExamples/test_seq_index_inbound1.rs --visitors RangeBoundsVisitor,QuantifierVisitor,ModularFlattenerVisitor,RecursionVisitor,FunctionInlineVisitor,RevealVisitor,LambdaVisitor,CollectionsVisitor --bound 5 --print-failed
// // proof is "incorrect" so SMarTPeek Fails! 
