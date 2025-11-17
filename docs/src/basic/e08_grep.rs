use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{ App, Arg };

pub fn grep() {
    let args = App::new("grep-x")
        .version("1.0.0")
        .about("Searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("the pattern to search for")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true)
        )
        .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    
    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    
    for x in reader.lines() {
        let line = x.unwrap();

        // Regex.find() 接收的参数类型为 str 型引用，所以需要传入 &line
        match re.find(&line) {
            Some(_) => println!("Found match: {}", line),
            None => (),
        }
    }
}
