// 使用 <A> 或者 <T> 作为类型的代号
// 类型参数，比如：<i32>

struct A; // 具体类型 A

struct S(A); // 具体类型 S

struct Generic<T>(T); // 泛类型 Generic

// 同样的规则也是使用与函数
fn _reg_fn(_s: S) {}

fn _gen_fn_a(_s: Generic<A>) {} // 不是泛型函数

fn _gen_fn_i32(_s: Generic<i32>) {} // 不是泛型函数

fn generic<T>(_s: Generic<T>) {} // 因为 Generic<T> 之前有 <T>，所以是关于泛型「T」的函数

impl generic<S> {}

// 需要标出 <T> 表示泛型
impl <T> generic<T> {}

fn main() {
    let _b = S(A);

    let _a = Generic(A);
    let _c = Generic('a');
    let _s = Generic("Hello World!");
    let _n = Generic(6);

    generic::<char>(Generic('a'));
}
