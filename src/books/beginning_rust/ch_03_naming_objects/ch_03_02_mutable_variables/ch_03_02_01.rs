/*
  Naming Objects
      3.2. Mutable Variables
*/

pub fn fn_03_02_01() {
    println!("-------------------------------------------------- 01");
    // 默认使用 let 声明的是一个 immutable variable，
    // 需要使用 let mut 来声明一个 mutable variable

    let mut i: u32 = 9;
    println!("{}", i); // 9
    i = 99;
    println!("{}", i); // 9
}
