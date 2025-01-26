use vstd::*;
use vstd::prelude::*;

verus! {






fn triangle(n:u8) -> (f: u8)
    requires n >= 0,
        n < 10
    ensures 0 <= f <= (256-n)
{
    // return 0;
    if n == 0 {
        return 0;
    } else {
        let ff = triangle(n-1);
        assert(0 <= ff <= (256-n-1));
        return ff;
        // return n + triangle((n - 1));
    }
}
// 

fn main() {
    
    let val = triangle(5);
    // assert(triangle(5) == 15);
}


}