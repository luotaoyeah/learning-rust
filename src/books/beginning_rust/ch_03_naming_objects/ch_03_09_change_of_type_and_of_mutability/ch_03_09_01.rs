/*
  Naming Objects
      3.9. Change of Type and of Mutability
*/

pub fn fn_03_09_01() {
    println!("-------------------------------------------------- 01");
    // 在同一个作用域，可以使用同一个变量名，重复声明多个不同类型的变量
    // 这种特性称之为 variable shadowing
    // 新的同名变量声明之后，前面的变量依然存在，但是不能再被访问到

    let mut n01: i32 = 9;
    n01 = 99;
    println!("{}", n01); // 99

    let n01: bool = true;
    println!("{}", n01); // true
}
