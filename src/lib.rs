extern crate clap;
extern crate regex;
extern crate colored;

use colored::*;
use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use regex::{Regex, Match};


/// Struct containing configuration for the search algorithm
pub struct Config {
    file_content: String,
    regex: Regex
}


impl Config {
    pub fn new() -> Config {

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

        Config {
            file_content: file_content,
            regex: regex
        }
    }
}


pub struct MatchInformation {
    words: Vec<String>,
    line_number: usize,
    line: String
}


impl MatchInformation {

    pub fn new(words: Vec<String>,
        line_number: usize,
        line: &str) -> MatchInformation {

        MatchInformation {
            words: words,
            line_number: line_number,
            line: String::from(line).clone()
        }
    }
}


/// Implementation of the search algorithm
pub fn do_search(params: &Config) -> Vec<MatchInformation>{

    let mut results = Vec::new();

    for (line_number, line) in params.file_content.lines().enumerate() {

        let mut matched_words = Vec::new();

        for m in params.regex.find_iter(&line[..]) {

            matched_words.push(String::from(m.as_str()));
        }

        if matched_words.is_empty() == false {

            let match_info = MatchInformation::new(matched_words, line_number, &line);
            results.push(match_info);
        }
    }

    results
}


/// Prints a line describing a match
pub fn print_match(match_result: MatchInformation) {

    for word in match_result.words {

        // let pref = String::from(&match_result.line[0..(boundary.0));
        // let inf = String::from(m.as_str());
        // let postf = String::from(&line[m.end()..]);
    }

    println!("{}: ", match_result.line_number);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_matches() {
        let file_content = "Lorem ipsum dolor sit amet,
consetetur sadipscing elitr,
sed diam nonumy eirmod tempor invidunt ut labore
et dolore magna aliquyam erat, sed diam voluptua.";

        let regex = Regex::new("li").unwrap();

        let cfg = Config {
            file_content: String::from(file_content),
            regex: regex
        };

        assert_eq!(
            vec!["consetetur sadipscing elitr,", "et dolore magna aliquyam erat, sed diam voluptua."],
            do_search(&cfg).iter().map(|m| m.line.clone()).collect::<Vec<String>>()
        );
    }
}
