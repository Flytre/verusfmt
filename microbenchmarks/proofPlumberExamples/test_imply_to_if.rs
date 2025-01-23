use vstd::prelude::*;

verus! {

    fn test_imply_to_if(b: bool) -> (ret: u32) 
    ensures 
      b ==> ret == 2 && !b ==> ret == 1,
{
    let mut ret: u32 = 1;
    if b {
        ret = ret + 1;
    }  
    assert(b ==> ret == 2);
    ret
}  


fn main() {}

}

// verifies -- no issues :) 