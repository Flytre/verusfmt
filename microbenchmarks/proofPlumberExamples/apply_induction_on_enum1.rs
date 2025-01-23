use vstd::prelude::*;

verus! {

    spec fn sequence_is_sorted(s: Seq<int>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
    }
    
    spec fn sequences_ordered_at_interface(seq1: Seq<int>, seq2: Seq<int>) -> bool {
        if seq1.len() == 0 || seq2.len() == 0 {
            true
        } else {
            seq1.last() <= seq2[0]
        }
    }
    
    #[is_variant] #[derive(PartialEq, Eq)]
    enum Tree {
        Nil,
        Node { value: i64, left: Box<Tree>, right: Box<Tree> },
    }
    impl Tree {
        spec fn view(&self) -> Seq<int>
            decreases self
        {
            match *self {
                Tree::Nil => seq![],
                Tree::Node { value, left, right } => left@.add(seq![value as int]).add(right@),
            }
        }
    
        spec fn is_sorted(&self) -> bool
            decreases self
        {
            match *self {
                Tree::Nil => true,
                Tree::Node { value, left, right } => {
                    &&& sequences_ordered_at_interface(left@, seq![value as int])
                    &&& sequences_ordered_at_interface(seq![value as int], right@)
                    &&& left.is_sorted()
                    &&& right.is_sorted()
                }
            }
        }
    }
    
    
    proof fn sorted_tree_means_sorted_sequence(tree: Tree)
        requires
            tree.is_sorted(),
        ensures
            sequence_is_sorted(tree@),
        decreases tree
    {
    }
    
    fn main() {}

    
}

// cargo run microbenchmarks/proofPlumberExamples/apply_induction_on_enum1.rs --visitors RangeBoundsVisitor,RecursionVisitor,RecursiveDatatypeVisitor --bound 5 --print-failed