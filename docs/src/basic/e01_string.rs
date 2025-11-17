// 代码可复制到 Rust 的 Playground 中感受
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024
fn fn01() {
    // 1. 多行字符串，注意开头的双引号后面是反斜杠
    // 为了避免意外的空格，内容需要顶格写，不要缩进
    // 结尾的双引号要紧跟字符串的结尾，不然最后会多出一个空行
    let one = "\
The quick brown
fox jumps over
the lazy dog";
    let mut lineNum: usize = 1;

    // 2. 遍历多行字符串，这段语法和 Python 几乎无差别
    for line in one.lines() {
        // 验证是否包含目标字符串
        if line.contains("a") {
            println!("[x] #{} {}", lineNum, line);
        } else {
            println!("[ ] #{} {}", lineNum, line);
        }

        lineNum += 1;
    }

    // 3. 遍历多行字符串的第二种方式，除了每行的数据，还能拿到每行的下标
    // 如果没有使用到 i 下标，则需要以下划线开头标记 i，记作 _i
    for (i, line) in one.lines().enumerate() {
        // 在行内使用了 if else 表达式，可以写变量的地方，同时也可以写表达式，因为变量也是一种表达式
        println!("[{}] #{} {}", if line.contains("q") { "x" } else { " " }, i + 1, line);
    }
}
