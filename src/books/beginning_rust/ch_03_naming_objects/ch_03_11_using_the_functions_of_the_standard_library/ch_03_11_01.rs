/*
  Naming Objects
      3.11. Using the Functions of the Standard Library
*/

pub fn fn_03_11_01() {
    println!("-------------------------------------------------- 01");
    // rust 标准库中提供的功能，不需要额外引入，可以直接使用

    // 调用函数有两种语法方式，procedural style 和 object-oriented style
    println!("{}", str::len("rust"));
    println!("{}", "rust".len());
}
