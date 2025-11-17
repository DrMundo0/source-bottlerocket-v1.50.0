// 定义结构体 MockFile，相当于 Java 中的类，可存放一组相关的字段
// #[derive(Debug)] 表示该类型实现了 std::fmt::Debug trait，可在 println! 宏中，使用 {:?} 占位符进行打印
#[derive(Debug)]
struct MockFile
{
    name: String,
    // 单字节值列表
    data: Vec<u8>,
}

pub fn startup() {
    // 使用字面量语法创建一个 MockFile 实例
    let f1 = MockFile {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    // 根据 f1 的引用去拿他的 name 字段
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("f1: {:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
