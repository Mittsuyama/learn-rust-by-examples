#![allow(dead_code)]
// 以闭包作为参数的函数
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

// 以闭包作为返回值的函数
// 由于 Rust 函数只能返回具体（非泛型）的类型，而匿名的闭包是未知类型的，所以只有使用 impl trait 来返回一个闭包
// Fn 和 FnMut trait 的闭包与普通函数没有什么区别
// FnOnce 则需要一点改动
fn create_closure() -> impl Fn() {
    let text = "this is a clouser".to_owned();
    // 需要使用 move 转移所有权，因为这个函数退出时，变量会被丢弃，形成悬垂指针
    move || println!("{}", text)
}

fn main() {
    // 闭包在输入和返回类型上可以自动推导
    // 而输入变量名必须指明
    let clouser_annotated = |i: i32| -> i32 { i + 1 };
    let clouser_inferred = |i| i + 1;
    let param = 5;
    println!("clouser_annotated = {}", clouser_annotated(param));
    println!("clouser_inferred = {}", clouser_inferred(param));
    let one = || 1;
    println!("one = {}", one());
    println!("------------------------------------------------------------------------");

    // 闭包的变量捕获
    let color = "green";
    let print = || println!("color = {}", color);
    print();
    print();

    // 捕获可变量变量
    let mut count = 1;
    // 闭包前标上 mut 表示闭包中捕获了一个可变变量
    let mut inc = || {
        count += 1;
        println!("count = {}", count);
    };
    inc();
    inc();

    let movable = Box::new(20);
    let consume = || {
        println!("movable = {:?}", movable);
        // 可复制类型会复制一份给闭包，而不可复制类型则会移动（move）到闭包中
        std::mem::drop(movable);
    };
    consume();

    let haystack = vec![1, 2, 3];
    // 使用 move 强制闭包获得被捕获变量的所有权
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    // 当闭包被定义时，编译器会隐式地创建一个匿名类型的结构体，用来保存捕获的变量
    // 并为这个结构体实现函数功能（Fn，FnMut，FnOnce 其中一种）
    // 所以闭包作为函数参数，就要求用法必须是泛型的，但是只要指明是是上述三种 trait 中的哪一种即可
    // 同时，满足 trait 约束的另一个函数，也能作为参数传入函数
    let clouser = || {
        // 通过引用捕获变量，此时推断闭包是 Fn
        println!("say {}!", greeting);

        // 可变引用，推断闭包是 FnMut
        farewell.push_str("!!!");
        println!("say {}", farewell);

        // 调用 drop 需要获得所有权，推断闭包是 FnOnce
        // 此时的闭包才能满足 apply 的 FnOnce 的约束
        std::mem::drop(farewell);
    };
    // 调用闭包
    apply(clouser);
    // 满足
    let double_it = |x| x * 2;
    println!("double it = {}", apply_to_3(double_it));
}
