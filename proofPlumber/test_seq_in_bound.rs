use vstd::seq::*;
use vstd::seq_lib::*;
use vstd::prelude::*;

verus! {

  // proof fn test_seq_in_bound() {
  //     let s1 = Seq::new(6, |i: int|
  //       if i == 0 { 10 }
  //       else if i == 1 { 20 }
  //       else if i == 2 { 30 }
  //       else if i == 3 { 45 }
  //       else if i == 4 { 55 }
  //       else { 70 }
  //     );
  //     let s2 = s1.filter(|x: int| x < 40);
  //     assert forall|i: int| s2[i] < 40 by{};
  // }

  // proof fn test_seq_in_bound() {
  //     let s1 = Seq::new(6, |i: int|
  //       if i == 0 { 10 }
  //       else if i == 1 { 20 }
  //       else if i == 2 { 30 }
  //       else if i == 3 { 45 }
  //       else if i == 4 { 55 }
  //       else { 70 }
  //     );
  //     let s2 = s1.filter(|x: int| x < 40);
  //     assert forall|i: int| 0 <= i && i < s2.len() implies s2[i] < 40 by {}
  //     assert forall|i: int| s2[i] < 40 by{};
  // }


  proof fn test_seq_in_bound_manual() {
    let s1 = Seq::new(6, |i: int|
      if i == 0 { 10 }
      else if i == 1 { 20 }
      else if i == 2 { 30 }
      else if i == 3 { 45 }
      else if i == 4 { 55 }
      else { 70 }
    ); 
    let s2 = s1.filter(|x: int| x < 40);
    // assert(0 < s2.len() ==> s2[0] < 40);
    // assert(1 < s2.len() ==> s2[1] < 40);
    // assert(2 < s2.len() ==> s2[2] < 40);
    // assert(3 < s2.len() ==> s2[3] < 40);
    // assert(4 < s2.len() ==> s2[4] < 40);
    // assert(5 < s2.len() ==> s2[5] < 40);
    // assert(6 < s2.len() ==> s2[6] < 40);
    // assert(7 < s2.len() ==> s2[7] < 40);

    // --> this is missing b/c there is no lhs to determine bound 
      // need some sort of case rule for seq (or other collection types, where indexing needs len bound)

    // assert(s2[0] < 40);
    // assert(s2[1] < 40);
    // assert(s2[2] < 40);
    // assert(s2[3] < 40);
    // assert(s2[4] < 40);
    // assert(s2[5] < 40);
    // assert(s2[6] < 40);
    // assert(s2[7] < 40);
    // --> this doesnt work b/c index out of bounds

    // assert forall|i: int| s2[i] < 40 by{};
}


  fn main(){}
}