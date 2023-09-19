/// 获取元组中元素的值
fn learn_tuple() {
    let a = (12, 23.4, 'c', false, "hello", "world".to_string());
    // 使用数组下边获取元素数据
    let b = a.0;
    println!("index 0={}", b);
    // 使用元组解构
    let (b, c, ..) = a;
    println!("b={},c={}", b, c);
}

/// 元组整体和元组中元素的所有权
fn learn_tuple_owner() {
    /*
     * 元组中每一个元素都实现了Copy Trait,因此整个元组可以认为实现了Copy Trait
     *
     * https://doc.rust-lang.org/std/marker/trait.Copy.html
     *
     * impl<T: Copy> Copy for (T₁, T₂, …, Tₙ)
     *
     * 因此把变量赋值给另一个变量不会发生所有权转移
     */
    let a = (12, 23.4, 'c', false, "hello");

    let b = a;
    let d = a; // 这里不会报错
    println!("{:?}", b);
    println!("{:?}", d);

    /*
     * 当元组中有元素未实现Copy Trait 时，元组整体也没有实现Copy ，会发生所有权转移
     */
    let c = (12, false, "hello", "world".to_string());
    //let e =c;// 把一个没有实现Copy 的变量赋值给另一个变量，会发生所有权转移，此时c 会move
    // let f=c;// 使用已move 的变量，错误

    /*获取元组中没有实现Copy 的元素，Copy 中的元素会发生移动 */
    let e = c.0;
    let f = c.0; // 这里不会报错，因为i32 类型实现了Copy ,赋值给另一个变量时拷贝一个副本
    println!("{}", e);
    println!("{}", f);
    // String 类型没有实现Copy
    let g = c.3; // world 这个元素会发生移动
                 // let h=c.3;
                 // 部分元素移动，造成整体元组不能使用
                 //println!("{:?}",c);
}

/// tuple 引用
fn learn_tuple_browing() {
    let c = (12, false, "hello", "world".to_string());
    // 引用，不会发生所有权移动
    let a = &c;
    let d = &c;
    if a == d {
        println!("a==d:{}", a == d)
    }
    println!("{}", a.0);
    println!("{}", a.3); // rust .自动解引用
    println!("{}", a.3);

    // 解构引用,解构引用使用ref
    let (.., ref s) = c; // 这里使用的是引用，因此String 类型的元素没有移动
    let n = s; // 引用
    println!("{:?}", c);
    println!("{:?}", s);
    println!("{:?}", n);
}

/// 元组作为参数
fn learn_tuple_as_fn_params(a: (i32, f64)) {
    println!("index=0,{}", a.0);
    println!("index=1,{}", a.1);
}

/// 使用type 关键字定义类型别名，简化难记的类型或者过长的类型
fn learn_tuple_params1(a: (i32, f64, char), b: (i32, f64, char), c: (i32, f64, char)) {}

type SimpleTuple = (i32, f64, char);
fn learn_tuple_params2(a: SimpleTuple, b: SimpleTuple, c: SimpleTuple) {}

/// 使用元组返回多个值
fn learn_tuple_as_return() -> (i32, f64) {
    (13, 45.0)
}

/// 交换两个同类型数据
fn learn_tuple_swap<T>(a: T,b: T) -> (T, T) {
    return (b, a);
}



fn main() {
    learn_tuple();
    learn_tuple_owner();
    learn_tuple_browing();
    learn_tuple_as_fn_params((12, 23.9));
    // 元组作为返回值，解析同元组处理一致
    let (a, b) = learn_tuple_as_return();
    println!("a={},b={}", a, b);
    let mut a = 3;
    let mut b=5;
    println!("a={},b={}", a, b);
    (a,b)=learn_tuple_swap(a, b);
    println!("a={},b={}", a, b);
}
