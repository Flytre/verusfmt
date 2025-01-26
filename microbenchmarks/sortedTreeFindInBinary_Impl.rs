#[allow(unused_imports)]
use prelude::*;
#[allow(unused_imports)]
use seq::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use vstd::*;

verus! {

#[is_variant]
#[derive(PartialEq, Eq)]  // TODO(utaal): Structural is not implemented for Box
enum Tree {
    Nil,
    Node { value: i64, left: Box<Tree>, right: Box<Tree> },
}

impl Tree {
    spec fn view(&self) -> Seq<int>
        decreases self,
    {
        match *self {
            Tree::Nil => seq![],
            Tree::Node { value, left, right } => left@.add(seq![value as int]).add(right@),
        }
    }

    spec fn is_sorted(&self) -> bool
        decreases self,
    {
        match *self {
            Tree::Nil => true,
            Tree::Node { value, left, right } => {
                &&& sequences_ordered_at_interface(left@, seq![value as int])
                &&& sequences_ordered_at_interface(seq![value as int], right@)
                &&& left.is_sorted()
                &&& right.is_sorted()
            },
        }
    }// #[verifier::proof] fn sorted_tree_means_sorted_sequence(&self)
    // TODO(utaal): is self being Spec too restrictive?

}

spec fn sequences_ordered_at_interface(seq1: Seq<int>, seq2: Seq<int>) -> bool {
    if seq1.len() == 0 || seq2.len() == 0 {
        true
    } else {
        seq1.last() <= seq2[0]
    }
}

spec fn sequence_is_sorted(s: Seq<int>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

// TODO: change the default for --multiple-errors
// we can have --jon-mode :p
// TODO: shall multiple errors in the same method be sorted?
proof fn sorted_tree_means_sorted_sequence(tree: Tree)
    requires
        tree.is_sorted(),
    ensures
        sequence_is_sorted(tree@),
    decreases tree  // guessed by Dafny ,
{
    // // reveal_with_fuel(sorted_tree_means_sorted_sequence, 3); // TODO(utaal) ICE revealing current method with fuel panics in AIR
    // if let Tree::Node { left, right, value: _ } = tree {
    //     sorted_tree_means_sorted_sequence(*left);  // guessed by Dafny
    //     sorted_tree_means_sorted_sequence(*right);  // guessed by Dafny
    // }
}

#[is_variant]
#[derive(Eq, PartialEq, Structural)]
enum TreeSortedness {
    Unsorted,
    Empty,
    Bounded(i64, i64),
}


fn find_in_binary_tree(tree: &Tree, needle: i64) -> (ret: bool)
    requires
        tree.is_sorted(),
    ensures
        ret == tree@.contains(needle as int),
    decreases tree,
{
    match tree {
        Tree::Nil => false,
        Tree::Node { left, value, right } => {
            if needle == *value {
                // assert(tree@[left@.len() as int] == needle);  // trigger
                true
            } else if needle < *value {
                true
            } else {
                let ret = find_in_binary_tree(right, needle);
                proof {
                    if ret {
                        let idx = choose|idx: int| 0 <= idx < right@.len() && right@[idx] == needle;
                        // assert(tree@[left@.len() + 1 + idx] == needle);  // trigger
                    } else {
                        // sorted_tree_means_sorted_sequence(**left);
                    }
                }
                ret
            }
        },
    }
}

fn main() {
}

} // verus!
