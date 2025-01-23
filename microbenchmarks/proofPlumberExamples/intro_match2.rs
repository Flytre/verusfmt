use vstd::prelude::*;

verus! {

    enum Movement {
        Up(u32),
        Down(u32),
    }
    
    spec fn is_good_move(m: Movement, a: int) -> bool {
        match m {
            Movement::Up(v) => v > a,
            Movement::Down(v) => v > 100,
        }
    }
    
    proof fn good_move(m: Movement)
    {
        assert(is_good_move(m, 100));
    }
    fn main() {}

}

//cargo run microbenchmarks/proofPlumberExamples/intro_match2.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor,RevealVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
