/*
  Naming Objects
      3.4. Uninitialized Variables
          variable must be initialized
*/

pub fn fn_03_04_03() {
    println!("-------------------------------------------------- 01");
    // 无论是 mutable variable 还是 immutable variable，
    // 在使用之前，都必须进行初始化，否则编译报错

    let i01: u32;
    let mut i02: u32;
}
