use vstd::seq::*;
use vstd::seq_lib::*;
use vstd::prelude::*;

verus! {

    #[derive(PartialEq, Eq, Clone)] 
    pub enum List {
        Nil,
        Node { value: int, next: Box<List>},

    }

    impl List {
        spec fn view(&self) -> Seq<int>
            decreases self,
        {
            match *self {
                List::Nil => seq![],
                List::Node { value, next } => next@.add(seq![value as int]),
            }
        }
    
    }

    spec fn sequence_is_sorted(s: Seq<int>) -> bool 
    {
        forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
    }

    spec fn list_is_sorted(list:List) -> bool
    {
        sequence_is_sorted(list@)
    }

    use List::*;
    proof fn adding_large_to_end_is_sorted(list:List, i:int) -> (s:Seq<int>)
        requires
            // list.maxDepth3(),
            list_is_sorted(list),
            list@[list@.len()-1] < i,
        ensures 
            sequence_is_sorted(s),
    {
        let new_node = List::Node{value: -1 , next: Box::new(Nil)};
        (list@ + new_node@)
        
    }


    fn main(){}

}