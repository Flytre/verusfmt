use vstd::seq::*;
use vstd::seq_lib::*;
use vstd::prelude::*;

verus! {


  proof fn test_seq_in_bound_manual() {
    let s1 = Seq::new(6, |i: int|
      if i == 0 { 10int }
      else if i == 1 { 20 }
      else if i == 2 { 30 }
      else if i == 3 { 45 }
      else if i == 4 { 55 }
      else { 70 }
    ); 
    let s2: Seq<int> = s1.filter(|x: int| x < 40);

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
  }


  fn main(){}
}