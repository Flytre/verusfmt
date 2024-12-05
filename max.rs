#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
use set::*;
#[allow(unused_imports)]
use prelude::*;
use multiset::*;

verus! {



fn max(v:Vec<u32>) -> (max:u32)
    requires 
        v.len() > 0,
    ensures 
        exists |i:int| 0 <= i <= v.len() && v[i] == max,
        forall |i:int| 0 <= i < v.len() ==> v[i] <= max,
{
    let mut i = 0;
    let mut max:u32 = v[0];
    while i < v.len()
        invariant
            // i >= 0,
            // i <= v.len(),
            // forall |j:int| 0 <= j < i ==> v[j] <= max,
            // exists |j:int| 0 <= j <= i && v[j] == max,
    {
        if(v[i] > max){
            max = v[i];
            // assert(max == v[i as int]);
        }
        i = i + 1;
    }
    // assert(false);
    return max;
}


fn main() {


    
}

} // verus!
