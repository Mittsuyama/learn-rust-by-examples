use std::fmt::Display;

//  要求掺入的参数 T 必须实现 Display trait
fn _printer<T: Display>(t: T) {
    println!("t = {}\n", t);
}

struct Point {
    x: f64,
    y: f64,
}

trait PointTrait {
    fn get_value(&self) -> f64;
}

impl PointTrait for Point {
    fn get_value(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// 要求实现了 PointTrait 的泛型
fn print_value<T: PointTrait>(t: T) {
    println!("value = {:?}", t.get_value());
}

// 多重约束
fn _multi_bound<T: PointTrait + Display>(_t: T) {}

// where 分句
fn _where_sentence_function<A, B, C>(_a: A, _b: B, _c: C) -> i32 where
    A: PointTrait + Display,
    B: Display,
    C: PointTrait {
    0
}

fn main() {
    let point = Point { x: 3.0, y: 4.0 };
    print_value(point);
}

