/// 数组定义，在rust 中数组长度是在编译时就确定了
///
/// 数组不仅包括类型，还有长度
///
/// 数组类型定义格式 \[T;length\]
///
fn learn_array() {
    let a: [i32; 4] = [0, 0, 0, 0];
    println!("{:?}", a);
}

/// 获取数组长度
///
fn learn_array_len() {
    let a: [i32; 4] = [0, 0, 0, 0];
    println!("{}", a.len());
}

/// 读取数组数据
/// 
fn learn_array_read(){
    let a: [i32; 4] = [0, 1, 2, 1];
    println!("{}",a[0]);
    println!("{}",a[2]);
}
/// join 拼接，但是元素必须是实现了Join 的
fn learn_array_string() {
    let a = ["0", "0", "0", "0"];
    let b = a.join("");
    println!("{:?}",b);//"0000"
}
fn main() {
    learn_array();
    learn_array_len();
    learn_array_string();
    learn_array_read();
}
