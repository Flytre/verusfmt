# Types of Visitors

##  Recursive Visitor

Affected Code Example:

```lisp
(defun factorial (n)
	(if (<= n 1)
	1
	(* n (factorial (- n 1))))
```

The same way we loop unroll, we need to unroll the recursion. SMT solvers have a hard time reasoning about recursion.


## Function Inline Visitor

Affected Code Example:

```lisp
(defun add-two (x)
  (+ x 2))
(add-two 5)
```

In this case, we know add-two is called with argument 5. We can pass in the argument 5 and inline it in a special version of the function so the SMT solver has to only reason about a finite example rather than all possible numbers


##  Loop Unroller

Affected Code Example:

```lisp
(do ((i 1 (+ i 1)))
    ((> i 10))
  (print i))
```

Its harder to reason about invariants in loops due to the number of iterations being possible infinite. We can unroll the loop into regular control flow to finitize.


## Quantifier Visitor

There exists / forall are a special case of verification-aware loops. We need to make sure these are handled too.



## Order interactions

### Scenario 1: Is Prime

Reference Code:

```rust
spec fn divides(factor: nat, candidate: nat) -> bool
    recommends
        1 <= factor,
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}

assert(is_prime(5))
```

There's a few orders we can handle the visitors here:

Option A:

1. Inline `is_prime_5`
2. Quantifier Unroll the `forall`
3. Inline each of the `divides` calls

Option B:
1. Assume an upper bound for `is_prime` and pr-emptively unroll the `forall`
2. Inline `is_prime_5`
3. Inline each of the `divides` calls

Logically, the second makes more sense since 

a) it retains an order on the visitors
b) it doesn't require intelliogently parsing the forall condition


This begs the question of whether we can always call Quantifier Unroll first and then Inline.

Is there a counter example? 

I argue not. Assume the code is pure: then we can always assume some bound on the quantifier to finitize its sample space. Following that we can always just inline as necessary.

The problem is if the bounds on the quantifier are hard to estimate, but that can only be put to the test by trying real programs. In general, most functions will operate on the space [0..n), as simple is more common than complex, so this likely only problematic for edge cases. 

### Function Inlining and Loop Unrolling

Reference code:
```rust

fn binary_search(v: &Vec<u64>, k: u64) -> (r: usize)
    requires
        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        exists|i: int| 0 <= i < v.len() && k == v[i],
    ensures
        r < v.len(),
        k == v[r as int],
{
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    while i1 != i2
        invariant
            i2 < v.len(),
            exists|i: int| i1 <= i <= i2 && k == v[i],
            forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
			func_call(v[i1]) # added for demo purposes
    {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    i1
}

assert(binary_search([1, 4, 9, 16], 9) == 2);

```

Initial Observations:

1. func_call(v[i1]) can only be inlined after both the call to binary search is inlined and after the loop is unrolled
2. intelligently loop unrolling based on an inlined input would be painful. dynamically understanding this is log(len(v)) is complex at verification time
3. guessing an unroll count based on speed while preserving sufficient behavioral bounds would be easy. Inlining after that would be easy.

Conclusion: In this specific example it makes sense to inline AFTER loop unrolling. Furthermore, we generalize this conclusion due to the evident complexity of picking an intelligent amount of times to loop unroll based on some given input -- my 376 is rusty but this is probably undecidable?

Logically, loop unrolling is similar to quantifier unrolling. So it makes sense that they have similar behavior.

### Function inlining and Recursive Calls

Reference Code:
```
spec fn func_call(n: nat) -> nat
{
	0
}

spec fn triangle(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat) + func_call(n)
    }
}

assert(triangles(5) == 15);
```

Thesis: Since recursion and iteration are fundamentally the same, they should have near-identical handling

Again, func call is artifically added to increase example complexity:

Initial Observations:

1. func_call(n) cannot be inlined until n is known: that would  be via recursion unrolling OR functrion inlining
2. function inlining is a form of recursion expansion here: it would go expand 5, then 4, then 3, etc. up to 0. however, if n is large, this will be slow and memory heavy

Conclusion: expand triangle from 0..x, and then if triangle(n) is called with a value <= x, use the expanded recusive value

Aka order:
1. Recursion Calls Expansion
2. Inline

This is consistent with the iterative case





