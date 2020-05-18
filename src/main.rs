use proc_macro_hack::proc_macro_hack;
extern crate proc_one;
extern crate proc_two;

/// Add one to an expression.
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
use proc_one::one;
#[proc_macro_hack]
use proc_two::two;

fn main() {
    let hi: &'static str = one!("hi");

    assert_eq!("hi", hi);
}
