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

// IMPL is correct But proof fails (post condition)
// Impl check -- success
// Proof check -- success
// Bound = 5


fn indexUpTo(n:u32) -> (f: Vec<u32>)
    requires n > 0,
    ensures f.len() == n, 
             f[0] == 0,
             n > 1 ==> f[n-1] != 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    assert(v.len() == 1);
    assert(v[0] == 0);
    while(i < n)
        invariant i > 0,
            v.len() == i,
            v[0] == 0,
            i <= n,
            
    {
        v.push(i);
        i = i + 1; 
        
    }
    return v;
}





fn main()

{

}

}