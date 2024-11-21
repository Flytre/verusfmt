use vstd::seq::*;
use vstd::seq_lib::*;
use vstd::prelude::*;

verus! {

    spec fn seq_bounded_by_length(s1: Seq<int>) -> bool 
    {
        forall|i:int| (0 <= i && i < s1.len())  ==>  (0 <= s1.index(i) && s1.index(i) < s1.len())
    }

    spec fn long_seq(s1: Seq<int>) -> bool {
        s1.len() > 20
    }

    spec fn seq_is_long_and_bounded_by_length(s1: Seq<int>) -> bool {
        seq_bounded_by_length(s1) && long_seq(s1)
    }

    proof fn test()
    {
        let mut ss: Seq<int> = Seq::empty();
        ss = ss.push(0);
        ss = ss.push(1);
        assert(seq_is_long_and_bounded_by_length(ss));
    }


    // spec fn seq_bounded_by_length(s1: Seq<int>) -> bool 
    // {
    //     forall|i:int| (0 <= i && i < s1.len())  ==>  (0 <= s1.index(i) && s1.index(i) < s1.len())
    // }


    // spec fn seq_bounded_by_length_finite(s1: Seq<int>) -> bool 
    // {
    //     // forall|i:int| (0 <= i && i < s1.len())  ==>  (0 <= s1.index(i) && s1.index(i) < s1.len())
    //     &&& (0 <= s1.index(0) && s1.index(0) < s1.len())
    //     &&& (0 <= s1.index(1) && s1.index(1) < s1.len())
    //     &&& (0 <= s1.index(2) && s1.index(2) < s1.len())
        

    // }


    // spec fn long_seq(s1: Seq<int>) -> bool {
    //     s1.len() > 10
    // }

    // spec fn seq_is_long_and_bounded_by_length(s1: Seq<int>) -> bool {
    //     seq_bounded_by_length(s1) && long_seq(s1)
    // }

    // proof fn test()
    // {
    //     let mut ss: Seq<int> = Seq::empty();
    //     ss = ss.push(0);
    //     ss = ss.push(1);
    //     ss = ss.push(2);
    //     ss = ss.push(3);
    //     ss = ss.push(4);
    //     ss = ss.push(5);
    //     ss = ss.push(6);
    //     ss = ss.push(7);
    //     ss = ss.push(8);
    //     ss = ss.push(9);
    //     ss = ss.push(10);
    //     // assert(long_seq(ss));
    //     // assert(seq_bounded_by_length(ss));

    //     // assert(seq_bounded_by_length_finite(ss));
    //     assert(seq_is_long_and_bounded_by_length(ss));
    // }




    fn main(){}
}