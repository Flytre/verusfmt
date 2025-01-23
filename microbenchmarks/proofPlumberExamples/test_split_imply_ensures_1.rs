use vstd::prelude::*;

verus! {

fn test_split_imply_ensures(b: bool) -> (ret: u32)
    ensures 
      b ==> ret == 2
{
    let mut ret: u32 = 1;
    if b {
        ret = ret + 1;
    }
    ret
}  


fn main() {}

}
  //verifies already  