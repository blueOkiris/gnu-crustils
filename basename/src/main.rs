/*
 * Author: Dylan Turner (original basename author: David MacKenzie)
 * Description:
 * - Recreation of gnu basename
 * - That means it strips a directory and suffix from a file name
 */

mod args;
mod ops;

use crate::{
    args::BasenameInputs,
    ops::perform_basename
};

fn main() {
    let inputs = BasenameInputs::get();
    for name in inputs.names {
        print!("{}", perform_basename(&name, &inputs.suffix, inputs.zero));
    }
}


