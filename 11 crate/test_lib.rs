pub fn function() {
    println!("function in test_lib!");
}

fn private_function() {
    println!("private_function in test_lib!");
}

pub fn call() {
    println!("call test_lib's private_function:");
    private_function();
    println!("end call.");
}
// 使用 rustc --crate-type=lib lib.rs 编译库文件时，会在文件前加上 lib 前缀
// 即编译成：liblib.rlib 文件
// 也可以用 crate-name 属性覆盖