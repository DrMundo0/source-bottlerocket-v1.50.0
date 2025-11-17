#![allow(unused_variables)]

use std::ffi::c_void;

// 为 File 类型定义别名 String
type File = String;

fn open(file: &mut File) -> bool {
    true
}

fn close(file: &mut File) -> bool {
    true
}

// 允许未使用的函数
#[allow(dead_code)]
fn read(file: &mut File, save_to: &mut Vec<u8>) {
    unimplemented!();
}

fn main() {
    let mut file = File::from("");
    open(&mut file);
    // read(&mut file, vec![]);
    close(&mut file);
}
