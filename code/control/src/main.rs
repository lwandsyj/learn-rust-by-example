fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    
    // 转成相同类型
    
    if a < (b as i32) {
      println!("Ten is less than one hundred.");
    }
  }
 