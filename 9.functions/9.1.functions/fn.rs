#![allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

// 方法在 impl 中定义
impl Point {
    // 这是静态方法，一般用于构造器
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    // 下面是实例方法
    // &self 是 self: &self 的语法糖
    fn transport(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    fn get_distance(&self, a: Point) -> f64 {
        ((self.x - a.x) * (self.x - a.x) + (self.y - a.y) * (self.y - a.y)).sqrt()
    }
}

fn main() {
    // 静态方法用双冒号使用
    let origin = Point::origin();
    let mut point = Point::new(2.0, 3.0);
    point.transport(1.0, 1.0);
    println!(
        "The distance between the point and the origin = {}",
        point.get_distance(origin)
    );
}
