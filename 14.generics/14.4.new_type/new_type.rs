// new type 惯用法（为不同种类的数据分别定义新的类型）
// 保证在编译时，给程序提供的都是正确的类型

use std::fmt;

struct Hours(i32);

struct Minute(i32);

impl Hours {
    pub fn to_minute(&self) -> Minute {
        Minute(&self.0 * 60)
    }
}

impl Minute {
    pub fn to_hour(&self) -> Hours {
        Hours(&self.0 / 60)
    }
}

impl fmt::Display for Hours {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} hours", self.0)
    }
}

impl fmt::Display for Minute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} minutes", self.0)
    }
}

fn main() {
    let hours = Hours(15);
    let minute = Minute(5434326);
    println!("hours to minutes = {}", hours.to_minute());
    println!("minutes to hours = {}", minute.to_hour());
}
