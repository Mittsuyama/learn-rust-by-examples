fn destroy_box(c: Box<i32>) {
    println!("Box: {}", c);
    // 得到 c 的所有权，输出并销毁
}

fn main() {
    // x 是一个栈分配的整型
    let x = 6i32;
    // 将 x 复制到 y，并没有发生所有权转移
    let y = x;

    println!("x is {}, and y is {}", x, y);

    // a 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);
    // 将 a 的指针地址复制给 b，虽然两者都指向同一个数据，但是 b 现在拥有所有权
    let b = a;
    // 取得 b 的所有权并销毁
    destroy_box(b);
}
