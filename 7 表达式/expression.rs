/**
 * Rust 由语句组成，有两种语句类型
 * 1. 声明/绑定变量
 * 2. 表达式 + 分号
 * 
 */

fn main() {
    let x = 10;
    //  代码块也是表达式，在语句中可以充当右值（r-value）
    let y = {
        let num1 = 10;
        let num2 = 20;
        num1 + num2
    };
    let z = {
        // 表达式 + 分号返回单元值 ()
        2 * x;
    };

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}