// 定义结构体 MockFile，相当于 Java 中的类，可存放一组相关的字段
// #[derive(Debug)] 表示该类型实现了 std::fmt::Debug trait，可在 println! 宏中，使用 {:?} 占位符进行打印
#[derive(Debug)]
struct MockFile
{
    // 文件名
    name: String,

    // 文件内容，Vec<u8> 单字节类型的列表
    data: Vec<u8>,
}

pub fn startup() {
    // 使用字面量语法创建一个 MockFile 实例
    let mut f1 = MockFile {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    // 初始化 vec，填入三个值
    f1.data.push('a' as u8);
    f1.data.push('\n' as u8);
    f1.data.push('b' as u8);

    // 根据 f1 的引用去拿他的 filename
    let f1_name = &f1.name;
    // data length
    let f1_length = &f1.data.len();

    println!("f1: {:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
