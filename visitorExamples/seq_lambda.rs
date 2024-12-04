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

    
    assert(s2[0]  < 40);
  }


  fn main(){}
}