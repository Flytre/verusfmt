#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {


    fn sum(value: nat) {
	let mut result = 0;
	for i in 0..value {
	    result += i;
	}
	result
    }


    fn main()

    {
        let n = 10;
	let r = sum(n);
	assert(r == 45);
    }

}
