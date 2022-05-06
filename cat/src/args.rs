/*
 * Author: Dylan Turner
 * Description: Handle argument stuff for basename recreation
 */

use clap::{
    Arg, Command, ArgMatches, crate_version
};

#[derive(Clone)]
pub struct CatInputs {
    pub names: Vec<String>,
    pub suffix: Option<String>,
    pub zero: bool
}

impl CatInputs {
    // Helper to get argmatches which will be parsed
    fn read_args() -> ArgMatches {
        Command::new("basename")
            .version(crate_version!())
            .author("Dylan Turner <dylantdmt@gmail.com>")
            .long_about(
                "Concatenate file(s) to standard output.\n\
                \n\
                With no <files>, or when <files> is -, read standard input"
            ).arg(
                Arg::new("show-all")
                    .short('A')
                    .long("show-all")
                    .takes_value(false)
                    .help("equivalent to -vET")
            ).arg(
                Arg::new("number-nonblank")
                    .short('b')
                    .long("number-nonblank")
                    .takes_value(false)
                    .help("number nonempty output lines, overrides -n")
            ).arg(
                Arg::new("little-e")
                    .short('e')
                    .takes_value(false)
                    .help("equivalent to -vE")
            ).arg(
                Arg::new("show-ends")
                    .short('E')
                    .long("show-ends")
                    .takes_value(false)
                    .help("display $ at end of each line")
            ).arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .takes_value(false)
                    .help("number all output lines")
            ).arg(
                Arg::new("squeeze-blank")
                    .short('s')
                    .long("squeeze-blank")
                    .takes_value(false)
                    .help("suppress repeated empty output lines")
            ).arg(
                Arg::new("little-t")
                    .short('t')
                    .takes_value(false)
                    .help("equivalent to -vT")
            ).arg(
                Arg::new("show-tabs")
                    .short('T')
                    .long("show-tabs")
                    .takes_value(false)
                    .help("display TAB characters as ^I")
            ).arg(
                Arg::new("show-nonprinting")
                    .short('v')
                    .long("show-nonprinting")
                    .takes_value(false)
                    .help("use ^ and M- notation, except for LFD and TAB")
            ).arg(
                Arg::new("files")
                    .required(true)
                    .takes_value(true)
                    .help("Comma separated list of files to use")
            ).get_matches()
    }

    pub fn get() -> Self {
        let args = Self::read_args();
        let mut names = Vec::new();
        for name in args.value_of("names").unwrap().split(",") {
            names.push(String::from(name));
        }
        Self {
            names,
            suffix: if args.is_present("suffix") {
                Some(String::from(args.value_of("suffix").unwrap()))
            } else {
                None
            }, zero: args.is_present("zero")
        }
    }
}
