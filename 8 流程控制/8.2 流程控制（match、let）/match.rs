#![allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RBG(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let number = 7;
    let result = match number {
        1 => "One",
        4 | 6 | 8 => "Two",
        2 | 3 | 5 | 7 => "Prime",
        9..=10 => "Nine to ten",
        _ => "other",
    };
    println!("Result: {}", result);

    // match 对各种类型的解构
    // 1. 元祖
    let pair = (0, 24238);
    match pair {
        (0, y) => println!("The first number is Zero, and the second is {}", y),
        (x, 0) => println!("The second number is Zero, and another is {}", x),
        _ => println!("There's no Zero"),
    }
    // 2. 枚举体
    // 同理也可以解构结构体
    let color = Color::RBG(102, 204, 255);
    match color {
        Color::Red => println!("The color is red."),
        Color::Blue => println!("The color is blue."),
        Color::Green => println!("The color is green."),
        Color::RBG(r, g, b) => println!("The color is rgb({}, {}, {})", r, g, b),
        _ => println!("The color is others"),
    }
    // 3. 指针和引用
    // Rust 中解引（*）和解构（&，ref，ref mut）不一样
    let reference = &4;
    match reference {
        // 注意这个 &
        // reference 此时是个引用
        &val => println!("这个需要用 & 来匹配：{}", val),
    }
    // 如果不想写 &，先解引然后直接匹配值
    match *reference {
        val => println!("先解引：{}", val),
    }
    // Rust 中可以用 ref 关键字修改复制过程
    // 使其变成引用
    let ref _is_reference = 3;
    // 在 match 中使用 ref
    let mut mut_reference = 3;
    match mut_reference {
        ref r => println!("ref r 的效果：{:?}", r),
    }
    match mut_reference {
        ref mut m => {
            // 先解引用获取值
            *m *= 7;
            println!("改变后的值：{:?}", mut_reference);
        },
    }

    // 在 match 中加入守卫来过滤分支
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Two variables with same value."),
        (x, y) if x + y == 0 => println!("They are oppposite numbers."),
        // 解构时也能用万能替代符
        // 但是 match 遇到第一个匹配的值后就会退出
        (x, _) if x % 2 == 0 => println!("The first number is odd."),
        _ => println!("Nothing happened."),
    }

    // 重新绑定（@ 符号）
    fn some_number() -> Option<i32> {
        Some(43)
    }
    match some_number() {
        // 使用 @ 符号来重新绑定，比如范围 n @ 1..=10，绑定后就能在 => 后直接使用了
        Some(n @ 42) => println!("Oh, it is {}", n),
        Some(n) => println!("A not interesting number: {}", n),
        _ => (),
    }

    // if let
    // 有时候只需要匹配一次，用 match 写起来不高效，所以提供了 if let 这样的语法
    let a = Color::Red;
    // 注意只有一个等号
    if let Color::Blue = a {
        println!("The color is Blue");
    } else {
        println!("It is some color i don't like");
    }
    // 注意，如果没有 #[derive(ParitialEq)] 属性，用 if Color::Red == a 是会报错的
    // if a == Color::Blue {
    //     println!("It is Blue!");
    // }
    // 错误信息：note: an implementation of `std::cmp::PartialEq` might be missing for `Color`
    // Rust 认为实例不具备可比性

    // while let
    // 解决 loop { match } 嵌套不美观的问题
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 10 {
            println!("The number is sooooooooooo large!");
            optional = None;
        } else {
            println!("The number is {}", i);
            optional = Some(i + 1);
        }
    }
}