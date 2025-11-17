# RustRover startup tool

Ubuntu 系统上的 RustRover 启动器

## 1. 安装系统依赖

自己编译的可执行文件如果不安装以下依赖，是不能执行的：

```bash
$ sudo apt update

$ sudo apt install -y libportaudio2 libqt5widgets5 pavucontrol firewall-config
```

安装完成后重启计算机

* ref / cant run executable (application/x-executable): https://askubuntu.com/a/1476421

## 2. 编译

毅 release 的方式编译项目：

```bash
$ cargo build --release
```

将 target/release 目录中的 StartupRustRover 文件复制到桌面，右键 Run as Program 即可运行 RustRover
