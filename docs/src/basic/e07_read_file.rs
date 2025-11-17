use std::fs::File;
use std::io::{ BufRead, BufReader };

// 演示如何读取文件内容
pub fn fn01() {
    // 读取自己项目的 Cargo.toml 文件
    let f = File::open("F:\\dev\\source-bottlerocket\\docs\\Cargo.toml").unwrap();

    // 使用带缓存的读取器，减少系统调用
    let mut reader = BufReader::new(f);

    // 存放文件中每一行的内容
    let mut line = String::new();

    loop {
        // 将行字符串变量的引用传入 read_line 函数
        // read_line 函数会返回一个 Result<usize> 类型，包含了该行的字节大小，用 unwrap 函数解包
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);

        // 收缩行字符串变量长度到0，防止之前行的内容遗留下来
        line.truncate(0);
    }
}

// 使用 BufReader 的 lines 函数，让读取文件每一行的内容更方便
pub fn fn02() {
    let f = File::open("F:\\dev\\source-bottlerocket\\docs\\Cargo.toml").unwrap();
    let reader = BufReader::new(f);

    for x in reader.lines() {
        let line = x.unwrap();
        // 比 fn01 更准确，每行都会少一个 byte
        println!("{} ({} bytes long)", line, line.len());
    }
}
