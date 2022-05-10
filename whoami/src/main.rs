/*
 * Author: Dylan Turner (original whoami author: Ruchard Mlynarik)
 * Description:
 * - Recreation of gnu whoami
 * - That means it shows the current user name
 */

use clap::{
    Command, crate_version
};
use users::{
    get_user_by_uid, get_current_uid
};

fn main() {
    // Allow --help and --version
    Command::new("whoami")
        .version(crate_version!())
        .author("Dylan Turner <dylantdmt@gmail.com>")
        .long_about(
            "Print the user name associated with the current effective user ID."
        ).get_matches();
    println!("{}", get_user_by_uid(get_current_uid()).unwrap().name().to_string_lossy())
}


