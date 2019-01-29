/*
  Naming Objects
      3.5. The Leading Underscore
*/

pub fn fn_03_05_01() {
    println!("-------------------------------------------------- 01");
    // 如果声明的变量未被使用，会有编译警告
    let i01: u32 = 9; // warning: unused variable: `i01`

    // 如果声明的变量未被使用，但是变量名是以 _ 开头，则不会有编译警告
    let _i02: u32 = 9;
}
