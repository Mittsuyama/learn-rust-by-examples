use std::mem;

// 别名
type Integer = i32;

fn main() {
    // Rust 不提供隐式转换，只能用「as」关键字进行显示转换
    let decimal = 65.435345;
    let integer = decimal as i32;
    println!("After changing: {:?}", integer);

    let x = 1.0f64;
    println!("size: {}", mem::size_of_val(&x));

    // vector 的类型推断，一开始并不知道内容是什么类型，从后面的代码中推断类型
    let mut vec = Vec::new();
    vec.push(0.2);
    println!("vector: {:?}", vec);

    let alias_variable: Integer = 4343.56654 as i32;
    println!("alias variable: {}", alias_variable);
}
