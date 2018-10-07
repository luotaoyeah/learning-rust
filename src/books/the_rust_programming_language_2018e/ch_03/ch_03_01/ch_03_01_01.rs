/*
   Common Programming Concepts
       Variables and Mutability
 */

pub fn fn_03_01_01() {
    let x = 5;
    println!("x: {}", x);

    /*
        // 使用 let 声明的变量默认是 immutable 的，不能重新赋值；
        x = 6; // [E0384]: cannot assign twice to immutable variable `x`
    */

    // 使用 mut 将变量标识为 mutable 的；
    let mut y = 5;
    println!("y: {}", y);
    y = 6;
    println!("y: {}", y);
}
