// Test that `by_move_binding @ pat_with_by_ref_bindings` is prevented even with promotion.
// Currently this logic exists in HAIR match checking as opposed to borrowck.

#![feature(bindings_after_at)]
#![feature(move_ref_pattern)]

fn main() {
    struct U;
    let a @ ref b = U; //~ ERROR borrow of moved value
}
