/*
  Naming Objects
      3.4. Uninitialized Variables
          initialize with another variable
*/

pub fn fn_03_04_02() {
    println!("-------------------------------------------------- 01");
    // 可以使用一个变量的值，来初始化另一个变量

    let i01: u32 = 9;
    let i02: u32;
    i02 = i01;
    println!("{}", i02); // 9
}
