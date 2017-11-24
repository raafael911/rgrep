extern crate clap;
extern crate regex;
extern crate colored;

use colored::*;
use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use regex::{Regex, Match};

struct CmdParams {
    file_content: String,
    regex: Regex
}


fn main() {

    let params = parse_params();

    do_search(&params);
}

fn do_search(params: &CmdParams) {

    for (line_number, line) in params.file_content.lines().enumerate() {

        for m in params.regex.find_iter(&line[..]) {

            print_match(&line, line_number, &m);
        }
    }
}

fn parse_params() -> CmdParams {

    let cmd_params = App::new("My Super own grep program")
                    .version("1.0")
                    .author("Kevin Neuenfeldt <kevin.neuenfeldt@posteo.de>")
                    .about("Does awesome things")
                    .arg(Arg::with_name("PATTERN")
                        .help("A Pattern to look for")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("FILE")
                        .help("A file to search for PATTERN")
                        .required(true)
                        .index(2))
                    .get_matches();

    let pattern = cmd_params.value_of("PATTERN").unwrap();
    let file_path = cmd_params.value_of("FILE").unwrap();

    let mut file = match File::open(file_path) {
        Err(_) => panic!(),
        Ok(f) => f
    };

    let regex = match Regex::new(pattern) {
        Err(_) => panic!(),
        Ok(r) => r
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Err(_) => panic!(),
        Ok(_) => ()
    };

    CmdParams {
        file_content: file_content,
        regex: regex
    }
}

/// Printsa line describing a match
fn print_match(line: &str, line_number: usize, m: &Match) {

    // TODO do not print a new line for each match in a line
    let pref = String::from(&line[0..m.start()]);
    let inf = String::from(m.as_str());
    let postf = String::from(&line[m.end()..]);

    print!("{}: ", line_number.to_string().cyan());
    print!("{}", pref);
    print!("{}", inf.bright_red());
    println!("{}", postf);
}
