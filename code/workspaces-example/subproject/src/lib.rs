pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// 使用pub 公开，方便其他模块调用
pub fn hello_world(){
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
