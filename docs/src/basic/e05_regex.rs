use regex::Regex;

// 演示如何引入第三方包，以及如何使用这个第三方包
// Rust 标准库中不包含对正则表达式的处理，需要引入 regex 包，Rust 中包称为 crate
// F:\> cargo add regex@1
// 引入 regex 包的 v1 版本，执行后可以观察 Cargo.toml 和 Cargo.lock 两个文件的变化

// 使用字符串自带的 contains() 函数精确匹配目标字符串的方式
pub fn fn01() {
    let quote = "\
01: The quick brown
02: fox jumps over
03: the la1zy dog";
    let needle = "a";

    for line in quote.lines() {
        if line.contains(needle) {
            println!("{}", line);
        }
    }
}

// 使用正则表达式来匹配
pub fn fn02() {
    let quote = "\
01: The quick brown
02: fox jumps over
03: the la1zy dog";
    // 声明一个正则表达式，并用 unwrap() 解包，如果成功，程序继续，如果失败，程序将崩溃
    let regex = Regex::new(r"\w").unwrap();

    for line in quote.lines() {
        let contains_substring = regex.find(line);

        match contains_substring {
            Some(_) => println!("{}", line),
            None => println!("Error")
        }
    }
}
