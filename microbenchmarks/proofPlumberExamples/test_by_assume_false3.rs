use vstd::prelude::*;

verus! {


proof fn f(a: u64, b: u64) { 
    assert((a & (a | b)) == a);
}

fn main() {}

}

// cargo run microbenchmarks/proofPlumberExamples/test_by_assume_false3.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor --bound 5 --print-failed
// FAILS b/c of bit-vector -- "Needs by(bit_vector);"
// -- "In Verus’s default prover mode, the definitions of these bitwise operators are not exported. To prove nontrivial facts about bitwise operators.."