#[allow(unused_imports)]
use vstd::prelude::*;
use vstd::{map::*, prelude::*, seq::*, set::*};

verus! {


    struct Foo {
        a: Seq<int>,
        b: Set<int>,
    }

    proof fn add_test(seq1: Seq<int>) {
         let seqA:Seq<int> = seq![10int, 11].add(seq![12, 13, 14]);
         let seqB: Seq<int> = seq![10int, 11, 12, 13, 14];
         assert(seqA =~= seqB);
         assert(seqA == Seq::new(seqA.len(), |i: int| seqA[i]));
        //  assert(match seqA {
        //     verus::vstd::seq::Seq => true,
        //     _ => false,
        // })
     }


    // proof fn ext_equal_struct() {
    //     let f1 = Foo { a: seq![1, 2, 3], b: set!{4, 5, 6} };
    //     let f2 = Foo { a: seq![1, 2].push(3), b: set!{5, 6}.insert(4) };
    //     // assert(f1 == f2);    // FAILS -- need to use =~= first
    //     assert(f1.a =~= f2.a);  // succeeds
    //     // assert(f1.b =~= f2.b);  // succeeds
    //     // assert(f1 == f2);  // succeeds, now that we've used =~= on .a and .b
    // }

    fn main()
    {
        
    }

}