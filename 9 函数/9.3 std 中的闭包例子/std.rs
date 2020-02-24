fn main() {
    let vec1 = vec![4, 5, 6];
    let vec2 = vec![1, 2, 3];
    // Iterator::any 是一个函数，只要迭代器中的一个元素满足谓词（即闭包的条件）则返回 true，否则 false
    // 注意 any 举出的是 &i32 类型的，所以要用 &x 进行匹配
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // Iterator::find 返回第一个满足谓词的元素
    // 由于 find 会把迭代器元素的引用传给闭包，迭代器本身是 &i32 类型的，所以实际传的是 &&i32 类型的值
    let array1 = vec![4, 5, 6];
    let array2 = vec![1, 2, 3];
    let mut iter = array1.iter();
    let mut into_iter = array2.into_iter();
    println!("Find 2 in vec2: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec1: {:?}", into_iter.find(|&x| x == 2));

    // 注意：
    // 使用 iter 和 into_iter 之后，数组的所有权会被转一走
}