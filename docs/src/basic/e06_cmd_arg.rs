use clap::{ App, Arg };
use regex::Regex;

// 演示如何从命令行接收参数
pub fn grep() {
    let args = App::new("grep-x")
        .version("0.1.0")
        .about("Searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true)
                .index(1)
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();

    // 用传入的参数实例化正则表达式
    let re = Regex::new(pattern).unwrap();

    let quote = "\
01: The quick brown
02: fox jumps over
03: the la1zy dog";
    
    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{:#?}", line),
            None => println!("{:#?}", line)
        }
    }
}