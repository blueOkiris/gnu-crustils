/*
 * Author: Dylan Turner (original whoami author: David MacKenzie)
 * Description:
 * - Recreation of gnu whoami
 * - That means it shows the current user name
 */

use clap::{
    Arg, Command, crate_version
};

fn main() {
    let args = Command::new("yes")
        .version(crate_version!())
        .author("Dylan Turner <dylantdmt@gmail.com>")
        .long_about(
            "Repeatedly output a line with all specified <message>, or 'y'"
        ).arg(
            Arg::new("message")
                .takes_value(true)
                .default_value("y")
                .help("Value to print forever")
        ).get_matches();

    let message = args.value_of("message").unwrap();
    loop {
        println!("{}", message);
    }
}


