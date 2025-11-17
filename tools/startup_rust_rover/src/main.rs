use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // RustRover 在 Ubuntu 上的安装目录
    let root = Path::new("/home/user01/Applications/RustRover-2025.1.5/bin");
    
    // 切换工作目录到 RustRover 的安装目录
    // https://doc.rust-lang.org/std/env/fn.set_current_dir.html
    let result = env::set_current_dir(root).unwrap();
    
    // 尝试执行 ls 命令，并拿到命令输出的内容
    // https://doc.rust-lang.org/std/process/struct.Command.html
    let output = Command::new("ls").output().expect("failed to execute process");

    // {:?} 格式化打印
    println!("Hello Rust :), {:?}, {}", result, root.display());

    if output.status.success() {
        println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
    } else {
        eprintln!("stderr: {}", String::from_utf8(output.stderr).unwrap());
        eprintln!("exit code: {:?}", output.status.code());
    }

    // 执行 RustRover 安装目录中的 rustrover.sh 启动脚本
    Command::new("bash").arg("./rustrover.sh").spawn().expect("failed to execute process");
}
