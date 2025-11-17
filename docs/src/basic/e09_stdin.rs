use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

// https://github.com/clap-rs/clap/tree/v2.34.0/examples
use clap::{ crate_authors, App, Arg };

// T 需实现 BufRead trait 和 Sized trait
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    // 遍历文件的每一行
    for line in reader.lines() {
        // 忽略错误，直接解包
        let line = line.unwrap();

        match re.find(&line) {
            Some(_) => println!("Found match: {}", line),
            None => (),
        }
    }
}

// 先进入编译结果所在目录
// $ cd docs/target/debug

// 查看帮助文档
// $ docs.exe --help

// 用短参数名设置要读取的文件名，和要匹配的正则
// $ docs.exe -i F:\dev\source-bottlerocket\docs\Cargo.toml -p ^version
pub fn startup() {
    let args = App::new("grep-x")
        .version("1.0.0")
        .author(crate_authors!()) // 读取 Cargo.toml 中的 authors 信息
        .about("Searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .short("p") // 短参数名
                .long("pattern") // 完整参数名
                .help("the pattern to search for")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("File to search")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(&pattern).unwrap();
    let input = args.value_of("input").unwrap();

    if input == "" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        process_lines(reader, re);
    }
}
