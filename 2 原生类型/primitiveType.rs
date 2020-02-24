use std::fmt;
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// 为 Matrix 增加 Display trait
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    println!("\n");
    let my_number = 1_0001;
    println!("为了增加可读性，可以在数字中间加入下划线，1_0001 = {:?}", my_number);
    println!("---------------------------------------------------------\n");
    
    let long_tuple = (1u8, 2i32, 3.3f32, 2.71828f64, ("another tuple", true));
    println!("输出一个元祖：{:?}", long_tuple);
    println!("---------------------------------------------------------\n");
    
    let pair = (1, true);
    println!("这是一个元祖：{:?}", pair);
    println!("替换元祖中的两个值：{:?}", reverse(pair));
    println!("---------------------------------------------------------\n");
    
    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("这是一个 Matrix：{:?}", matrix);
    println!("为 Matrix 增加一个 Display trai 之后的输出：\n{}", matrix);
    println!("transpose 之后：\n{}", transpose(matrix));
    println!("---------------------------------------------------------\n");
    
    // 数组和切片，数组：[T; siz]，切片：双字对象，头指针 + 长度
    let arr: [i32; 5] = [10; 5];
    println!("这是一个定长数组：{:?}", arr);
    println!("这个数组的长度：{:?}", arr.len());
    println!("数组所占内存大小：{:?}", mem::size_of_val(&arr));
    println!("将数组借用为 slice，并且指向其中一部分：{:?}", &arr[1..4]);
    println!("---------------------------------------------------------\n");
}