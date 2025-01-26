use vstd::prelude::*;
use vstd::seq::*;

verus! {

    spec fn seq_bounded_by_length(s1: Seq<int>) -> bool 
{
    forall|i:int| 0 <= i < s1.len()  ==>  0 <= s1.index(i) && s1.index(i) < s1.len()
}

spec fn long_seq(s1: Seq<int>) -> bool {
    s1.len() > 20
}

spec fn seq_is_long_and_bounded_by_length(s1: Seq<int>) -> bool {
    seq_bounded_by_length(s1) && long_seq(s1)
}

proof fn test()
{
    let mut ss: Seq<int> = Seq::empty();
    ss = ss.push(0);
    ss = ss.push(1);
    assert(seq_is_long_and_bounded_by_length(ss));
}

fn main() {}

}


//cargo run microbenchmarks/proofPlumberExamples/decompose_function_inline.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
