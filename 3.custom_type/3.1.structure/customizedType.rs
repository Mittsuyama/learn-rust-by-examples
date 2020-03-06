// 定义常量
#![allow(dead_code)]

static PROGRAM_LAGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

// 类 C 的结构体
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 元祖结构体（具名元组）
struct Pair(i32, f32);

// 单元结构体
struct Empty;

struct Point {
    x: f32,
    y: f32,
}

// 允许出现死代码，即可以出现未被使用的函数，结构体等
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[derive(Debug)]
struct Edge {
    aim: i32,
    cost: i32,
    last: i32,
}

fn main() {
    let peter = Person {
        name: "Peter",
        age: 18,
    };
    println!("Hello, this is Peter: {:?}", peter);

    let point = Point { x: 0.3, y: 0.4 };
    // 更新 Point 结构体值
    let _new_point = Point { x: 0.5, ..point };

    let _new_pair = Pair(1, 0.1);
    let _empty = Empty;

    let e = Edge { aim: 1, cost: 1, last: 1 };
    // 解构的只能放在最后
    let another_edge = Edge {
        aim: 2,
        ..e
    };
    println!("another_edge: {:?}", another_edge);
}
