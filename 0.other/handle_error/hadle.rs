// 1. 先看下面的代码
fn _get_file_contents() {
    let path = "/tmp/data";
    println!("data: {:?}", read_file(path));
}

fn _read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
// 如果 path 路径发生问题，比如文件没有找到
// 程序就会发生崩溃

use std::io::Error as OtherError;
use std::fmt;

// 为了避免 panic 的发生，修改成如下程序
fn modified_read_file() {
    let path: &str = "/tmp/data";
    match read_file(path) {
        Ok(file) => println!("file contents: {}", file),
        Err(e) => println!("error: {}", e),
    }
}

// Rust 中 Result 是一个 enum 枚举对象
fn read_file(path: &str) -> Result<String, OtherError> {
    std::fs::read_to_string(path)
}

// 2. 自定义 Error 类型，需要实现
//     - std::fmt::Display 的 trait
//     - std::fmt::Debug 的 trait，一般添加 #[derive(Debug)] 即可
//     - std::error::Error 的 trait，根据自身 error 级别是否覆盖 std::error::Error 的 source 方法
// 在 source() 中，如果当前是低级 Error，没有子 Error，则返回 None
// 如果包含子 Error，需要返回子 Error：some(error)，需要覆盖该方法

use std::error::Error;

#[derive(Debug)]
struct ChildError;

// 自定义 Error
#[derive(Debug)]
struct CustomError {
    err: ChildError,
}

impl fmt::Display for ChildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Children Error!")
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "costumed Error!")
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}

impl std::error::Error for ChildError {}

fn get_super_error() -> Result<(), CustomError> {
    Err(CustomError { err: ChildError })
}

fn test_costumed_error() {
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e);
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("no Error"),
    }
}

// 3. 我们假设要读入文件 -> 转成 utf8 -> 再转成 i32
// 需要嵌套三层 match，写起来非常的恶心
// 要利用自定义 Error 转换：From

#[derive(Debug)]
enum FileToUtf8ToIntError {
    ParseIntError(std::num::ParseIntError),
    UTF8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}

impl std::error::Error for FileToUtf8ToIntError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            FileToUtf8ToIntError::IoError(ref e) => Some(e),
            FileToUtf8ToIntError::ParseIntError(ref e) => Some(e),
            FileToUtf8ToIntError::UTF8Error(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for FileToUtf8ToIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            FileToUtf8ToIntError::IoError(ref e) => e.fmt(f),
            FileToUtf8ToIntError::ParseIntError(ref e) => e.fmt(f),
            FileToUtf8ToIntError::UTF8Error(ref e) => e.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for FileToUtf8ToIntError {
    fn from(s: std::num::ParseIntError) -> Self {
        FileToUtf8ToIntError::ParseIntError(s)
    }
}

impl From<std::io::Error> for FileToUtf8ToIntError {
    fn from(s: std::io::Error) -> Self {
        FileToUtf8ToIntError::IoError(s)
    }
}

impl From<std::str::Utf8Error> for FileToUtf8ToIntError {
    fn from(s: std::str::Utf8Error) -> Self {
        FileToUtf8ToIntError::UTF8Error(s)
    }
}

fn another_read_file(path: &str) -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn to_utf8(v: &[u8]) -> std::result::Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

fn to_u32(v: &str) -> std::result::Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

fn change_file_to_utf8_to_u32() -> std::result::Result<(), FileToUtf8ToIntError> {
    let path = "/tmp/data";
    let file = another_read_file(path)?;
    let utf8 = to_utf8(file.as_bytes())?;
    let u32 = to_u32(utf8)?;
    println!("content: {:?}", u32);
    Ok(())
}

fn main() {
    // 修改后的文件读写
    modified_read_file();
    // 自定义错误类型
    test_costumed_error();
    // 使用 from 读取文件并转换
    let _ = change_file_to_utf8_to_u32();
}
