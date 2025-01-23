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
      if i == 0 { 10int }
      else if i == 1 { 20 }
      else if i == 2 { 30 }
      else if i == 3 { 45 }
      else if i == 4 { 55 }
      else { 70 }
    ); 
    let s2 = s1.filter(|x: int| x < 40);

    // let mut s3 = seq![];
    // let mut x:int = 0;
    // if(x < s1.len()){
    //   if(s1[x] < 40){
    //     s3 = s3.push(s1[x]);
    //   }
    //   x = x + 1;
    // }
    // if(x < s1.len()){
    //   if(s1[x] < 40){
    //     s3 = s3.push(s1[x]);
    //   }
    //   x = x + 1;
    // }
    // if(x < s1.len()){
    //   if(s1[x] < 40){
    //     s3 = s3.push(s1[x]);
    //   }
    //   x = x + 1;
    // }
    // if(x < s1.len()){
    //   if(s1[x] < 40){
    //     s3 = s3.push(s1[x]);
    //   }
    //   x = x + 1;
    // }
    // if(x < s1.len()){
    //   if(s1[x] < 40){
    //     s3 = s3.push(s1[x]);
    //   }
    //   x = x + 1;
    // }
    // if(x < s1.len()){
    //   if(s1[x] < 40){
    //     s3 = s3.push(s1[x]);
    //   }
    //   x = x + 1;
    // }


    // assert(0 < s2.len() ==> s2[0] < 40);
    // assert(1 < s2.len() ==> s2[1] < 40);
    // assert(2 < s2.len() ==> s2[2] < 40);
    // assert(3 < s2.len() ==> s2[3] < 40);
    // assert(4 < s2.len() ==> s2[4] < 40);
    // assert(5 < s2.len() ==> s2[5] < 40);
    // assert(6 < s2.len() ==> s2[6] < 40);
    // assert(7 < s2.len() ==> s2[7] < 40);
    // assert(s3[0]  < 40);
    assert(s2[0]  < 40);

    // assert(forall|i: int| 0 <= i < s2.len() ==> s2[i] < 40);

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
////////////////////
// proof fn test() {
//   // TODO: seq! is currently defined via repeated pushes.
//   // This has sometimes led to Z3 timeouts, including in this test.
//   // It may be time for a more efficient definition of seq!
//   // (for example, via if/else, as shown below).
//   //let s1 = seq![10, 20, 30, 45, 55, 70];
//   let s1 = Seq::new(6, |i: int|
//       if i == 0 { 10 }
//       else if i == 1 { 20 }
//       else if i == 2 { 30 }
//       else if i == 3 { 45 }
//       else if i == 4 { 55 }
//       else { 70 }
//   );
//   let s2 = s1.filter(|x: int| x < 40);
//   let s3 = seq![90, 100];
//   let s4 = s3.filter(|x: int| x < 40);
//   // Test for successful broadcast of filter_lemma_broadcast
//   assert(forall|i: nat| i < s2.len() ==> s2[i as int] < 40);
//   // Test for successful broadcast of filter_distributes_over_add
//   assert((s1 + s3).filter(|x: int| x < 40) == (s2 + s4));
//   assert(s2[0] < 40);
//   // Test for successful broadcast of push_distributes_over_add
//   assert((s2 + s4).push(120) == s2 + s4.push(120));
// }


  fn main(){}
}