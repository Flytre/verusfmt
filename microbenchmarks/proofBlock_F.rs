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

struct S {
    a: u8,
    b: Ghost<int>,
}


    impl Clone for S {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl Copy for S {}

    impl S {
        fn equals(&self, rhs: &S) -> (b: bool)
            ensures
                b == (self.a == rhs.a),
        {
            self.a == rhs.a
        }

        fn addToB(&mut self, i:int)
            ensures
                self.b@ == old(self).b@ + i
        {
            proof{
                self.b@ = self.b@;
                assert(true);
            }

        }

        

    }

fn main() {
    
}

} // verus!
