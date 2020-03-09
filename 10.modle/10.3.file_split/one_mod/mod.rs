pub mod inner_mod;
// 只能这个文件访问 center_mod;
mod center_mod;

pub fn function() {
    println!("function in one_mod!");
}

pub fn call() {
    println!("call center_mod's function in one_mod!");
    center_mod::function();
    println!("call end!");
}