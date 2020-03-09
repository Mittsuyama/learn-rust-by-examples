// use 时，可以将一段路径绑定到一个新名字上，方便访问
use std::mem::drop as another_function;

// 模块的可见性
#[allow(dead_code)]
mod my_first_mod {
    // private 和 public
    fn private_fuction_in_my_first_mod() {}
    pub fn public_function_in_my_first_mod() {}
    fn use_private_function() {
        // 在模块中可以访问 private 项
        private_fuction_in_my_first_mod();
    }
    // 嵌套模块
    pub mod inner_mod {
        // pub(in path)，表示只在给定 path 中可见
        // path 只能是父模块或者祖先模块
        pub(in my_first_mod) fn functin_only_seen_in_father_mod() {}
        // self 仅自己可见
        pub(self) fn function_only_seen_in_this_mod() {}
        // super 仅父模块可见
        pub(super) fn function_only_seen_in_father_mod() {}
    }
    // crate 同一个 crate 可见
    pub(crate) fn function_only_seen_in_this_crate() {}
}

mod struct_mod {
    // 公有结构体 + 公有字段
    pub struct PublicStruct<T> {
        pub content: T,
    }
    // 公有结构带一个私有字段
    pub struct PublicStructPrivateContent<T> {
        _content: T,
    }
    // 给上面这个结构体加一个公有构造器
    impl<T> PublicStructPrivateContent<T> {
        pub fn new(content: T) -> PublicStructPrivateContent<T> {
            PublicStructPrivateContent {
                _content: content,
            }
        }
    }
}

fn main() {
    // 公有结构体能用字段名来构造
    let _pub_struct = struct_mod::PublicStruct { content: "Hello world!" };
    // 带有私有字段的结构体只能用公有构造器来构造
    let _public_struct_private_content = struct_mod::PublicStructPrivateContent::new("Hello another world!");
}