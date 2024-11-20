use vstd::seq::*;
use vstd::seq_lib::*;
use vstd::prelude::*;

verus! {

    enum Movement {
        Up(u32),
        Down(u32),
    }

    spec fn is_good_move(m: Movement) -> bool {
        match m {
            Movement::Up(v) => v > 108,
            Movement::Down(v) => v > 100,
        }
    }

    proof fn good_move(m: Movement)
    {
        // assert(is_good_move(m));
        let mm = Movement::Up(421);
        assert(is_good_move(mm))
        // assert(is_good_move(Movement::Up(420)));
        // assert(
        //     match Movement::Up(420){
        //         Movement::Up(v) => v > 100,
        //         Movement::Down(v) => v > 100,
        //     }
        // )
    }

    // enum Movement {
    //     Up(u32),
    //     Down(u32),
    // }

    // spec fn is_good_move(m: Movement) -> bool {
    //     match m {
    //         Movement::Up(v) => v > 100,
    //         Movement::Down(v) => v > 100,
    //     }
    // }

    // proof fn good_move(m: Movement)
    // {
    //     match m {
    //         Movement::Up(..) => assert(is_good_move(m)),
    //         Movement::Down(..) => assert(is_good_move(m)),
    //     };
    // }

    //// ----- intro match 2 ---- 

    spec fn is_good_move2(m: Movement, a: int) -> bool {
        match m {
            Movement::Up(v) => v > a,
            Movement::Down(v) => v > 100,
        }
    }
    
    proof fn good_move2(m: Movement)
    {
        let mm = Movement::Up(4210);
        assert(is_good_move2(mm, 1080));
    }

    /// ---- intro match 3 ----- 

    #[derive(PartialEq, Eq, Clone)] 
    #[is_variant]
    pub enum Message {
        Quit(bool),
        Move { x: i32, y: i32 },
        Write(bool),
    }

    spec fn message_well_formed(msg:Message) -> bool {
        match msg {
            Message::Quit(b) => !b,
            Message::Move{x, y} => x < y,
            Message::Write(b) => b,
        }
    }

    // fn update_msg(msg: Message) 
    //     requires message_well_formed(msg)
    // {
    //     let new_msg = match msg {
    //         Message::Quit(b) => Message::Quit(b),
    //         Message::Move{x, y} => Message::Move{x: 0, y: 1},
    //         Message::Write(b) => Message::Write(b),
    //     };

    //     assert(message_well_formed(new_msg));
    // }
    
    fn update_msg(msg: Message)
    requires
        match msg {
            Message::Quit(b) => !b,
            Message::Move { x, y } => (x == 0 && y == 5),
            Message::Write(b) => b,
        }
        {
                let new_msg = match msg {
                    Message::Quit(b) => Message::Quit(b),
                    Message::Move{x, y} => Message::Move{x: x+1, y: y-1},
                    Message::Write(b) => Message::Write(b),
                };
        
                assert(message_well_formed(new_msg));
            }



fn main() {}



}