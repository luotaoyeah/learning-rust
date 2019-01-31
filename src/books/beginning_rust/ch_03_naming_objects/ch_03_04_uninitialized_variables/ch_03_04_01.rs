/*
  Naming Objects
      3.4. Uninitialized Variables
*/

pub fn fn_03_04_01() {
    println!("-------------------------------------------------- 01");
    // 变量可以在声明的同时就进行初始化
    let i01: u32 = 9;
    println!("{}", i01);

    // 变量也可以先声明，再初始化
    let i02: u32;
    i02 = 9;
    println!("{}", i02);

    // 如果一个变量没有进行初始化就被使用，则会编译报错
    let i03: u32;
    /*
        println!("{}", i03); // [E0381]: borrow of possibly uninitialized variable: `i03`
    */
}
