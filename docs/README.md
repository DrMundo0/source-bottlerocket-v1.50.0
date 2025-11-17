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
> cargo build --release
```

https://doc.rust-lang.org/std/env/fn.set_current_dir.html
https://doc.rust-lang.org/std/process/struct.Command.html
