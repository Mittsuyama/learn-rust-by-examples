/**
* 属性是应用于某些模块、crate、项的元数据（metadata），可以用来：
* 1. 条件编译代码
* 2. 设置 crate 名称、版本和类型（二进制文件或库）
* 3. 禁用 lint（警告）
* 4. 启用编译器特性（宏，全局导入等）
* 5. 连接到非 Rust 语言的库
* 6. 标记函数作为单元测试
* 7. 标记函数作为基准测试的某个部分
*/

// 应用于整个 crate：#![crate_attribute]
// 仅仅模块或者项：#[item_attribute]（注意少了 !）

// 语法格式
// #[attribute(value1, value2)]

// 禁用由于未使用函数产生的警告
#[allow(dead_code)]
fn unused_function() {}

// crate_type 表明这是一个库文件（rustc 编译的时候就不需要再加上 --crate-type 标记了）
// crate_name 设置库的名称
// 但是注意由于很多工程使用 cargo，所以这两个选项的用处不是很大
// #![crate_type = "lib"]
// #![crate_name = "attribute"]

// 当这个系统不是 Linux 时才会编译（是某个系统去掉 not 即可）
#[cfg(not(target_os = "linux"))]
fn _not_linux_functio() {}

fn main() {
    if cfg!(tartget_os = "linux") {
        println!("this is linux!");
    } else {
        println!("this is not linux!");
    }
    _conditional_function();
}

// 用户自定义的条件必须要用 --cfg 来传给 rustc
// 否则报错：「cannot find function `_conditional_function` in this scope」
#[cfg(customed_condition)]
fn _conditional_function() {}