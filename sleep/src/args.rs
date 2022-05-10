/*
 * Author: Dylan Turner
 * Description: Handle argument stuff for basename recreation
 */

use std::time::Duration;
use clap::{
    Arg, Command, ArgMatches, crate_version
};

#[derive(Clone)]
pub struct SleepInputs {
    pub time: Duration
}

impl SleepInputs {
    // Helper to get argmatches which will be parsed
    fn read_args() -> ArgMatches {
        Command::new("sleep")
            .version(crate_version!())
            .author("Dylan Turner <dylantdmt@gmail.com>")
            .long_about(
                "Pause for <number> seconds. <number>> may contain a suffix.\n\
                Suffixes: 's' for seconds (default), 'm' for minutes, 'h' for hours,\n\
                Or 'd' for days.\n\
                <number> need not be an integer.\n\
                Given two or more arguments, pause for the amount of time specified by their sum"
            ).arg(
                Arg::new("number")
                    .required(true)
                    .takes_value(true)
                    .help("Single number or comma separated list of numbers to delay for")
            ).get_matches()
    }

    pub fn get() -> Result<Self, &'static str> {
        let args = Self::read_args();
        let mut numbers = Vec::new();
        for num_str in args.value_of("number").unwrap().split(",") {
            let time_scale: f64 = match num_str.chars().last().unwrap() {
                'd' => 60.0 * 60.0 * 24.0,
                'h' => 60.0 * 60.0,
                'm' => 60.0,
                's' => 1.0,
                '0' => 1.0,
                '1' => 1.0,
                '2' => 1.0,
                '3' => 1.0,
                '4' => 1.0,
                '5' => 1.0,
                '6' => 1.0,
                '7' => 1.0,
                '8' => 1.0,
                '9' => 1.0,
                _ => -1.0
            };
            if time_scale == -1.0 {
                return Err("Unknown suffix added!");
            }
            let num = num_str.split_at(num_str.len() - 1).0;
            numbers.push(String::from(num).parse::<f64>().unwrap() * time_scale);
        }
        
        Ok(Self {
            time: Duration::from_secs_f64(numbers.iter().sum())
        })
    }
}
