use vstd::prelude::*;

verus! {


enum Movement {
    Up(u32),
    Down(u32),
}

spec fn is_good_move(m: Movement) -> bool {
    match m {
        Movement::Up(v) => v > 100,
        Movement::Down(v) => v > 100,
    }
}

proof fn good_move(m: Movement)
{
    assert(is_good_move(m));
}
fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/intro_match1.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor,RevealVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
