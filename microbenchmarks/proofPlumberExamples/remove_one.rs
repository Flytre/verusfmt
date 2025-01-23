use vstd::prelude::*;

verus! {
proof fn remove_one_foo(x: nat)
    ensures
        x >= 0,
{
    assert(x >= 0);
}

proof fn remove_multiple_foo(x: nat)
    ensures
        x >= 0,
{
    assert(x >= 0);
    assert(x + x >= 0);
    assert(x * x >= 0) by (nonlinear_arith);
    assert(x + 3 >= 3);
}

proof fn preserve_necessary_foo(x: u32, y: u32)
    ensures
        x & y == y & x,
{
    assert(x & y == y & x) by (bit_vector);
}

proof fn assert_comment_success_proof_index(a: u16, offset: u16)
    requires
        offset < 16,
    ensures
        offset < 16,
{
    assert(offset < 16);
    assert(1 == 1);
    assert(15 < 16);
}

pub open spec fn fibo(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else if n == 1 { 1 }
    else { fibo((n - 2) as nat) + fibo((n - 1) as nat) }
}


proof fn lemma_fibo_is_monotonic(i: nat, j: nat)
    requires
        i <= j,
    ensures
        fibo(i) <= fibo(j),
    decreases j - i
{
    if i < 2 && j < 2 {
    } else if i == j {
    } else if i == j - 1 {
        reveal_with_fuel(fibo, 2);
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
    } else {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        lemma_fibo_is_monotonic(i, (j - 2) as nat);
    };
}

proof fn remove_autogen_asserts_fibo_lemma_fibo_is_monotonic(i: nat, j: nat)
    requires i <= j,
    ensures fibo(i) <= fibo(j),
    decreases j - i
{
    if i < 2 && j < 2 {
        assert(fibo(i) <= fibo(j));
    } else if i == j {
        assert(fibo(i) <= fibo(j));
    } else if i == j - 1 {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        assert(fibo(j) == fibo((j-1) as nat) + fibo((j-2) as nat));
        assert(fibo(i) <= fibo(j));
    } else {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        lemma_fibo_is_monotonic(i, (j - 2) as nat);
        assert(fibo(i) <= fibo(j));
    };
    assert(fibo(i) <= fibo(j));
}


proof fn remove_autogen_asserts_fibo2_lemma_fibo_is_monotonic(i: nat, j: nat)
    requires i <= j,
    ensures fibo(i) <= fibo(j),
    decreases j - i
{
    if i < 2 && j < 2 {
    } else if i == j {
    } else if i == j - 1 {
        assert(fibo(i) <= fibo((j - 1) as nat) ==> fibo(j) == fibo((j - 1) as nat) + fibo(
            (j - 2) as nat,
        ));
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        assert(fibo(j) == fibo((j-1) as nat) + fibo((j-2) as nat));
    } else {
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        lemma_fibo_is_monotonic(i, (j - 2) as nat);
    };
}


proof fn remove_autogen_asserts_fibo3_lemma_fibo_is_monotonic(i: nat, j: nat)
    requires
        i <= j,
    ensures
        fibo(i) <= fibo(j),
    decreases j - i,
{
    if i < 2 && j < 2 {
        assert(fibo(i) <= fibo(j));
    } else if i == j {
        assert(fibo(i) <= fibo(j));
    } else if i == j - 1 {
        assert(fibo(i) <= fibo((j - 1) as nat) ==> fibo(j) == fibo((j - 1) as nat) + fibo(
            (j - 2) as nat,
        ) ==> fibo(i) <= fibo(j));
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        assert(fibo(j) == fibo((j - 1) as nat) + fibo((j - 2) as nat) ==> fibo(i) <= fibo(j));
        assert(fibo(j) == fibo((j - 1) as nat) + fibo((j - 2) as nat));
        assert(fibo(i) <= fibo(j));
    } else {
        assert(fibo(i) <= fibo((j - 1) as nat) ==> fibo(i) <= fibo((j - 2) as nat) ==> fibo(i)
            <= fibo(j));
        lemma_fibo_is_monotonic(i, (j - 1) as nat);
        assert(fibo(i) <= fibo((j - 2) as nat) ==> fibo(i) <= fibo(j));
        lemma_fibo_is_monotonic(i, (j - 2) as nat);
        assert(fibo(i) <= fibo(j));
    };
    assert(fibo(i) <= fibo(j));
}


fn main() {}

}

//All verifies (7)
// remove_one
// remove_multiple
// preserve_necessary
// assert_comment_success
// remove_autogen_asserts_fibo
// remove_autogen_asserts_fibo2
// remove_autogen_asserts_fibo3