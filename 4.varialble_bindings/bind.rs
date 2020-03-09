fn main() {
    // 编译器会对没有使用过的变量加以警告，可以在变量前加下划线消除警告
    let _unused_variable = 3u32;

    // 可变变量
    let mut mutable_number = 1;
    mutable_number += 1;
    println!("After change: {:?}", mutable_number);

    // Rust 和 C 同样有变量作用域，{  } 包围的代码表示一个作用域
    // 但是 Rust 允许变量掩蔽
    let _be_covered_variable = 1;
    let _be_covered_variable = 'a';

    // Rust 同样可以先绑定再赋值，但不推荐这样的做法，容易产生「未定义行为」
}