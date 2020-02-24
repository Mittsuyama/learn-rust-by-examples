fn main() {
    // 要求：求所有平方是奇数且小于某个阈值的数的和
    // 设定一个阈值
    let upper = 1000;
    let sum: u32 = 
        (0..).map(|n| n * n)
             .take_while(|&n| n < upper)
             .filter(|&n| n % 2 == 1)
             .fold(0, |sum , i| sum + i);
    println!("sum = {}", sum);

    // 发散函数
    // 发散函数绝对不会返回，用 ! 进行标记
    // ! 还在试验阶段
    // let x: ! = panic!("This call will never return");

    // 还有其他用法
    let mut sum = 0;
    for index in 0..1000 {
        let additional: i32 = match index % 2 == 1 {
            true => index,
            // additional 的值的要求是 i32，而 continue 永远不会返回值
            // 所以也满足类型要求
            false => continue,
        };
        sum += additional;
    }
    println!("sum = {}", sum);
}