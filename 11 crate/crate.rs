/*
* 调用 rutsc some_file.rs 时，some_file.rs 会被当做「crate 文件」。
* 如果有调用 mod 的声明，则会把被导入的文件直接编译进此 crate 中
* 即 mod 不会被单独编译，crate 才会被编译
*/

// 导入其他的库
extern crate test_lib;

fn main() {
    println!("main process begin");
    test_lib::function();
    test_lib::call();
    println!("main process end");
}

// 使用如下语句编译
// rustc crate.rs --extern test_lib=libtest_lib.rlib;