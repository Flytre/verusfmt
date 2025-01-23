use vstd::prelude::*;

verus! {

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
    
  fn update_msg(msg: Message) 
      requires message_well_formed(msg)
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

//cargo run microbenchmarks/proofPlumberExamples/intro_match3.rs --visitors RangeBoundsVisitor,QuantifierVisitor,FunctionInlineVisitor,ModularFlattenerVisitor,RecursionVisitor,RevealVisitor --bound 5 --print-failed
// proof is "incorrect" so SMarTPeek Fails! 
