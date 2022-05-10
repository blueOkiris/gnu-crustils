/*
 * Author: Dylan Turner
 * Description: Handle argument stuff for basename recreation
 */

use clap::{
    Arg, Command, ArgMatches, crate_version
};

#[derive(Clone)]
pub struct BasenameInputs {
    pub names: Vec<String>,
    pub suffix: Option<String>,
    pub zero: bool
}

impl BasenameInputs {
    // Helper to get argmatches which will be parsed
    fn read_args() -> ArgMatches {
        Command::new("basename")
            .version(crate_version!())
            .author("Dylan Turner <dylantdmt@gmail.com>")
            .long_about(
                "Print names with any leading directory components removed\n\
                If specified, also remove a trailing SUFFIX\n\
                \n\
                Mandatory arguments to long options are mandatory for short options too"
            ).arg(
                Arg::new("multiple")
                    .short('a')
                    .long("multiple")
                    .takes_value(false)
                    .help("support multiple arguments and treat each as a NAME")
            ).arg(
                Arg::new("suffix")
                    .short('s')
                    .long("suffix")
                    .takes_value(true)
                    .help("remove a trailing SUFFIX; implies -a")
            ).arg(
                Arg::new("zero")
                    .short('z')
                    .long("zero")
                    .takes_value(false)
                    .help("end each output line with NUL, not newline")
            ).arg(
                Arg::new("names")
                    .required(true)
                    .takes_value(true)
                    .help("Comma separated list of names to remove prefix (and optionally suffix from)")
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
