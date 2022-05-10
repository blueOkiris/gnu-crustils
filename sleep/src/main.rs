/*
 * Author: Dylan Turner (original sleep author: Jim Meyering and Paul Eggert)
 * Description:
 * - Recreation of gnu sleep
 * - That means it delays for a specified amount of time
 */

mod args;

use std::thread::sleep;
use crate::args::SleepInputs;

fn main() {
    match SleepInputs::get() {
        Ok(inputs) => sleep(inputs.time),
        Err(msg) => println!("{}", msg)
    };
}


