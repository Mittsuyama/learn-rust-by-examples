struct Empty;
struct Null;

// T 泛类型的 trait
trait DoubleDrop<T> {
    // 接受一个额外参数
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // 此方法获取两个参数的所有权，并释放他们
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    
    // empty;
    // null;
    // 两个变量已经 drop 掉了
}
