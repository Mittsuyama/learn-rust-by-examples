/*
* 对一个工程中的 mod 进行文件分层
* root
* |----- one_mod
* |         |------- mod.rs
* |         |------- inner_mod.rs
* |         |------- center_mod.rs
* |----- another_mod.rs 
*/

// 导入 one_mod
// rust 会在查找 one_mod.rs 或者 one_mod/mod.rs 的文件，导入进来
mod one_mod;

fn function() {
    println!("function in another_mod!");
}

fn main() {
    function();
    one_mod::function();
    one_mod::call();
    one_mod::inner_mod::function();
}