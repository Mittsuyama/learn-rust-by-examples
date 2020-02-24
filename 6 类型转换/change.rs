/*
 * Rust 使用 trait 解决类型转换，一般会用 From 和 into 两个 trait
 * 还可以自定义类型转换机制
 * 也就是给自定义类型写 From 这个 trait
 * 此时可以免费获得 into
 * 
 * TryFrom 和 TryInto 提供了易出错的转换，所以返回 Result 类型
 * 
 * ToString 可以实现任何类型转字符串
 * 但是建议直接实现 fmt::Dsiplay 这个 trait 从而打印实例
 * 
 * FromStr 则是从字符串转换到给定类型，标准库中实现了很多这样的 trait
 * 
*/

use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EventNumber(i32);

impl TryFrom<i32> for EventNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(EventNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let my_str = "Hello World!!!";
    let my_string = String::from(my_str);
    println!("my_string: {}", my_string);

    let number = Number::from(30);
    println!("from: {:?}", number);
    // 注意要求制定转换目标的类型，此时编译器不能自动推断
    let num: Number = 5.into();
    println!("into: {:?}", num);

    // try 系列
    println!("First Result: {:?}", EventNumber::try_from(8));
    println!("Second Result: {:?}", EventNumber::try_from(-1));
    let result: Result<EventNumber, ()> = (-1i32).try_into();
    println!("try_into result: {:?}", result);

    // FromStr
    let num1: i32 = "5".parse().unwrap();
    let num2: i32 = "5".parse::<i32>().unwrap();
    println!("sum: {:?}", num1 + num2);
}
