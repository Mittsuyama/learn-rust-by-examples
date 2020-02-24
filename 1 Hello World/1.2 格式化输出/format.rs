use std::fmt;

#[derive(Debug)]
struct Container(i32);

#[derive(Debug)]
struct Box(Container);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
    gender: u8,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

#[derive(Debug)]
struct Path(Vec<i32>);

// 注意 ? 符号的使用
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let res = &self.0;
        for (index, item) in res.iter().enumerate() {
            // if 也是一个表达式
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} + {:.2}i", self.real, self.imag)
    }
}

fn main() {
    /*
     * 打印输出由 std::fmt 中的宏来处理
     * format! 输出到字符串
     * print! / println! 输出到控制台（std::stdout）
     * eprint! / eprintln! 输出到标准错误（std::stderr）
     */
    // {} 会被替换成任意类型变量
    println!("One Chinese month has {} days.", 31);

    // {} 中使用位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 命名参数
    println!(
        "{subject} {verb} {object}.",
        subject = "A bird",
        object = "a song",
        verb = "is singing"
    );
    println!("--------------------------------");

    // 在 :（冒号）后面指定特殊格式
    println!(
        "This is binary number: {:b}, and this is hexadecimal: {:x}",
        5438978, 5438978
    );
    println!("--------------------------------");

    // 右对齐文本
    // 用空格对齐
    println!(
        "right-justify with space: {number:>width$}",
        number = 21,
        width = 10
    );
    // 用 0 对齐
    println!(
        "right-justify with zero: {number:>0width$}",
        number = 21,
        width = 10
    );
    println!("--------------------------------");

    // 保留小数点三位
    println!("PI = {:.3}", 3.1415926);
    println!("--------------------------------");

    // 格式化输出 struct
    println!("This is a Box struct: {:?}", Box(Container(17)));
    // {:#?} # 号表示美化打印
    println!(
        "This is a Person struct: {:#?}",
        Person {
            name: "Peter",
            age: 18,
            gender: 1
        }
    );
    println!("--------------------------------");

    let complex = Complex {
        real: 3.1415926,
        imag: 2.71828,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
    println!("--------------------------------");

    let v = Path(vec![1, 2, 3, 4]);
    println!("This is a vector: {}", v);
}
