use vstd::prelude::*;

verus! {
    enum Movement {
        Up(u32),
        Down(u32),
    }
    
    //added 'a'
    proof fn good_move(m: Movement, a: int)
    {
        match m {
            Movement::Up(v) => v > a,
            Movement::Down(v) => {
                let foo:int = 1;
                foo > 100
            },
        };
        assert(true);
    }
fn main() {}

}

// verifies already  (small changes - otherwise doesnt resolve)
