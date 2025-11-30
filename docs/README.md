```bash
# 打印 rustc 工具的版本信息
F:\> rustc --version

# 编译，生成可执行文件
F:\> rustc filename.rs

# 引入正则表达式包
F:\> cargo add regex@1

# 引入命令行参数包
F:\> cargo add clap@2

# 根据 Cargo.toml 拉取依赖
F:\> cargo build

# 打包为生产版本，会进行性能优化
$ cargo build --release
```

https://doc.rust-lang.org/std/env/fn.set_current_dir.html
https://doc.rust-lang.org/std/process/struct.Command.html

## Cross-compilation (交叉编译)

```bash

$ rustup target list
# x86_64-pc-windows-gnu
# x86_64-pc-windows-gnullvm
# x86_64-pc-windows-msvc

$ rustup target add x86_64-pc-windows-gnu

# error: linker `x86_64-w64-mingw32-gcc` not found
# 安装 MinGW 工具包
$ sudo apt install mingw-w64

# 进入工作目录，准备编译 docs 子项目
$ cd /home/user01/Documents/dev/source-bottlerocket-v1.50.0/docs/

# 编译 debug 版本的 Windows 平台包
$ cargo build --target=x86_64-pc-windows-gnu
# 编译后的结果在项目根的 source-bottlerocket-v1.50.0/target/x86_64-pc-windows-gnu/debug/ 目录中
```

* Cross-compilation: https://rust-lang.github.io/rustup/cross-compilation.html

## JSON 序列化
