/*
   Common Programming Concepts
       Shadowing
 */

pub fn fn_03_01_03() {
    /*
       使用 let 重复声明同名的变量，后面的变量会隐藏调之前声明的变量，称之为 shadowing；
     */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);

    /*
       shadowing 的时候，可以改变变量的类型；
     */
    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
