/**
 * Rust 的流程控制：
 * 1. if-else
 * 2. loop 循环
 * 3. while 循环
 */

fn main() {
    // Rust 的 if-else
    // 1. 判断语句不需要用括号包围
    // 2. 每个条件都返回一个代码块
    // 3. 所有的分支的返回值必须相同
    let n = 5;
    let x = 
        if n < 0 {
            println!("This number is smaller than zero.");
            -1
        } else if n < 10 {
            println!("This number is smaller than 10.");
            0
        } else {
            println!("This number is bigger than 10.");
            1
        };
    println!("Get the number: {:?}", x);

    // Rust 中的 loop 循环
    // 等价于 while true（但不建议用 while true，会有返回值问题）
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Now count is {}", count);
        }
        if count >= 10 {
            println!("Now count is {}", count);
            break;
        }
    }
    // 嵌套 loop 循环和标签
    let mut number = 1;
    'outer: loop {
        number += 1;
        'inner: loop {
            number += 3;
            if number > 10 {
                println!("the number is {}", number);
                break 'outer;
            }
        }
    }
    // loop 还能用 break 返回值
    let mut count = 0;
    let result = loop {
        count += 3;
        if count > 20 {
            break count * 5;
        }
    };
    println!("The result is {}", result);

    // while 循环类比于 if 语句

    // for in 语法可以遍历一个 iterator（迭代器）
    let mut sum = 0;
    // 用「=」 表示左闭右闭区间，没有则是左闭右开区间
    for item in 1..=100 {
        sum += item;
    }
    println!("sum = {}", sum);

    // 或者用一个函数使一个集合转换成迭代器
    let mut names = vec!["Alice", "Bob", "Cancy"];
    // iter() 只是单纯借用，iter_mut 则是能改变的借用，还有 into_iter，每次借用之后删除
    for item in names.iter_mut() {
        // 其他两种 for 不需要 * 取值
        *item = match item {
            &mut "Cancy" => "There is a name vec",
            _ => "Hello",
        };
    }
    println!("vec : {:?}", names);
}