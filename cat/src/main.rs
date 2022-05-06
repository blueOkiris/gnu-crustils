/*
 * Author: Dylan Turner (original cat author: Torbjorn Granlund)
 * Description:
 * - Recreation of gnu cat
 * - Concatenate files and print on standard output
 */

mod args;
mod ops;

use crate::{
    args::CatInputs,
    ops::perform_basename
};

fn main() {
    let inputs = CatInputs::get();
    for name in inputs.names {
        print!("{}", perform_basename(&name, &inputs.suffix, inputs.zero));
    }
}


